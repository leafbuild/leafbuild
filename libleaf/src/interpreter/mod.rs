use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::{collections::HashMap, ops::Deref, path::Path};

use lalrpop_util::ParseError;

use crate::interpreter::types::{resolve_executable_property_access, Executable};
use crate::{
    grammar::{self, ast::*, lexer::LexicalError, TokLoc},
    handle::Handle,
    interpreter::{
        diagnostics::{errors::*, push_diagnostic_ctx, DiagnosticsCtx, Location},
        types::{
            resolve_map_property_access, resolve_num_property_access, resolve_str_property_access,
            resolve_vec_property_access, TypeId, TypeIdAndValue,
        },
    },
};
use libutils::compilers::Compiler;
use libutils::{
    compilers::{
        cc::{get_cc, CC},
        cxx::{get_cxx, CXX},
    },
    generators::{ninja::*, *},
};

#[path = "diagnostics/diagnostics.rs"]
pub(crate) mod diagnostics;
pub(crate) mod ops;
pub(crate) mod types;

pub(crate) const DOCS_ROOT: &str = "https://leafbuild.gitlab.io/docs/";

pub struct EnvConfig {
    angry_errors_enabled: bool,

    error_cascade_enabled: bool,

    output_directory: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        Self {
            angry_errors_enabled: false,
            error_cascade_enabled: true,
            output_directory: "".to_string(),
        }
    }

    #[inline]
    pub fn set_angry_errors(&mut self, enabled: bool) -> &mut EnvConfig {
        self.angry_errors_enabled = enabled;
        self
    }

    #[inline]
    pub fn set_error_cascade(&mut self, enabled: bool) -> &mut EnvConfig {
        self.error_cascade_enabled = enabled;
        self
    }

    #[inline]
    pub fn set_output_directory(&mut self, output_directory: impl Into<String>) -> &mut EnvConfig {
        self.output_directory = output_directory.into();
        self
    }
}

pub(crate) enum Language {
    C,
    CPP,
}

pub(crate) struct EnvImut {
    diagnostics_ctx: DiagnosticsCtx,
    call_pools: CallPoolsWrapper,
    config: EnvConfig,
}

pub(crate) struct LangToolchainRules<T>
where
    T: Rule,
{
    preprocess: T,
    compile: T,
    link: T,
}

pub(crate) struct EnvMut {
    /// the current executable id we are at, universally unique
    exec_id: usize,

    /// the C compiler, if necessary
    cc: Option<CC>,
    /// the C++ compiler, if necessary
    cxx: Option<CXX>,

    executables: Vec<Executable>,
}

impl EnvMut {
    pub(crate) fn get_and_cache_cc(&mut self) -> &CC {
        if self.cc.is_none() {
            let cc = get_cc().expect("Cannot find CC");
            self.cc = Some(cc);
        }
        self.cc.as_ref().unwrap()
    }

    pub(crate) fn get_and_cache_cxx(&mut self) -> &CXX {
        if self.cxx.is_none() {
            let cxx = get_cxx().expect("Cannot find CXX");
            self.cxx = Some(cxx);
        }
        self.cxx.as_ref().unwrap()
    }
}

pub(crate) struct Env {
    imut: EnvImut,
    mut_: EnvMut,
}

impl Env {
    pub(crate) fn new(cfg: EnvConfig) -> Self {
        Self {
            imut: EnvImut {
                diagnostics_ctx: DiagnosticsCtx::new(
                    cfg.angry_errors_enabled,
                    cfg.error_cascade_enabled,
                ),
                call_pools: CallPoolsWrapper::new(),
                config: cfg,
            },
            mut_: EnvMut {
                exec_id: 0,
                cc: None,
                cxx: None,
                executables: vec![],
            },
        }
    }

    pub(crate) fn write_results(&self) -> Result<(), Box<dyn Error>> {
        let buf = PathBuf::from(self.imut.config.output_directory.clone());
        let ninja_file = buf.join("build.ninja");
        let f = File::create(ninja_file)?;

        let mut gen = NinjaGen::new();
        let cc_compile = gen.new_rule("cc_compile", NinjaCommand::new("cc $in -c -o $out"));
        let cc_link = gen.new_rule("cc_link", NinjaCommand::new("cc $in -o $out"));

        let cxx_compile = gen.new_rule("cxx_compile", NinjaCommand::new("c++ $in -c -o $out"));
        let cxx_link = gen.new_rule("cxx_link", NinjaCommand::new("c++ $in -o $out"));

        self.mut_.executables.iter().for_each(|exe| {
            let mut need_cxx_linker = false;

            let exe_args: Vec<NinjaRuleArg> = exe
                .get_sources()
                .iter()
                .filter_map(|src| {
                    let rl;
                    if CC::can_compile(src) {
                        rl = &cc_compile;
                    } else if CXX::can_compile(src) {
                        rl = &cxx_compile;
                        need_cxx_linker = true;
                    } else if CC::can_consume(src) || CXX::can_consume(src) {
                        return None;
                    } else {
                        println!(
                            "Warning: ignoring file '{}' while building executable '{}'",
                            src,
                            exe.get_name()
                        );
                        return None;
                    }
                    gen.new_target(
                        format!("{}.o", src),
                        rl,
                        vec![NinjaRuleArg::new(format!("../{}", src))],
                        vec![],
                    );
                    Some(NinjaRuleArg::new(format!("{}.o", src)))
                })
                .collect();
            gen.new_target(
                exe.get_name(),
                if need_cxx_linker { &cxx_link } else { &cc_link },
                exe_args,
                vec![],
            );
        });

        gen.write_to(f)?;

        Ok(())
    }
}

pub(crate) struct ProjectData {
    name: String,
}

pub(crate) struct ModuleData {
    name: String,
}

pub(crate) enum EnvFrameType {
    Workspace,
    Project(ProjectData),
    Module(ModuleData),
    Unknown, // default value, uninitialized
}

pub(crate) struct EnvFrame<'env> {
    env_ref: &'env EnvImut,
    env_mut_ref: &'env mut EnvMut,
    variables: HashMap<String, Variable<Box<dyn ValueTypeMarker>>>,
    env_frame_data: EnvFrameData,
    file_id: usize,
    fr_type: EnvFrameType,
}

impl<'env> EnvFrame<'env> {
    pub(crate) fn get_value_for_variable(
        &self,
        id: &str,
    ) -> Option<&Value<Box<dyn ValueTypeMarker>>> {
        match self.variables.iter().find(|&(var_name, _)| var_name == id) {
            Some(var) => Some(var.1.get_value()),
            None => None,
        }
    }

    #[inline]
    pub(crate) fn get_diagnostics_ctx(&self) -> &'env DiagnosticsCtx {
        &self.env_ref.diagnostics_ctx
    }

    #[inline]
    pub(crate) fn get_pools_wrapper(&self) -> &'env CallPoolsWrapper {
        &self.env_ref.call_pools
    }

    #[inline]
    pub(crate) fn get_variables_mut(
        &mut self,
    ) -> &mut HashMap<String, Variable<Box<dyn ValueTypeMarker>>> {
        &mut self.variables
    }

    #[inline]
    pub(crate) fn get_file_id(&self) -> usize {
        self.file_id
    }

    #[inline]
    pub(crate) fn new_executable(&mut self, name: String, sources: Vec<String>) -> &Executable {
        let id = self.env_mut_ref.exec_id;
        self.env_frame_data
            .exe_decls
            .push(Executable::new(id, name, sources));
        self.env_mut_ref.exec_id += 1;
        self.env_frame_data.exe_decls.last().unwrap()
    }
}

pub struct EnvFrameData {
    /// the executables that need to be built and are private
    exe_decls: Vec<Executable>,
    /// the executables accessible from the outer build system
    pub_exe_decls: Vec<Executable>,
}

pub(crate) struct EnvFrameReturns {
    exe_decls: Vec<Executable>,
    pub_exe_decls: Vec<Executable>,
}

impl EnvFrameData {
    pub(crate) fn empty() -> Self {
        Self {
            exe_decls: vec![],
            pub_exe_decls: vec![],
        }
    }
}

impl From<EnvFrameData> for EnvFrameReturns {
    fn from(r: EnvFrameData) -> Self {
        Self {
            exe_decls: r.exe_decls,
            pub_exe_decls: r.pub_exe_decls,
        }
    }
}

impl EnvFrameReturns {
    fn apply_changes_to_env(&self, env: &mut Env) {
        self.exe_decls
            .iter()
            .for_each(|exe| env.mut_.executables.push(exe.clone()));
        self.pub_exe_decls
            .iter()
            .for_each(|exe| env.mut_.executables.push(exe.clone()));
    }
}

struct EnvLibDecl {
    name: String,
    type_: EnvLibType,
    /// The path of the lib relative to the target directory of the current env
    path: String,
}

enum EnvLibType {
    Static,
    Shared,
}

struct EnvExeDecl {
    name: String,
    /// The path of the lib relative to the target directory of the current env
    path: String,
}

pub(crate) struct Variable<T>
where
    T: ValueTypeMarker,
{
    name: String,
    value: Value<T>,
}

impl<T> Variable<T>
where
    T: ValueTypeMarker + Sized,
{
    pub(crate) fn new(name: String, value: Value<T>) -> Self {
        Self { name, value }
    }
    #[inline]
    pub(crate) fn get_value(&self) -> &Value<T> {
        &self.value
    }
    #[inline]
    pub(crate) fn get_value_mut(&mut self) -> &mut Value<T> {
        &mut self.value
    }
}

pub(crate) trait ValueTypeMarker {
    fn stringify(&self) -> String;
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>>;
    fn get_type_id(&self) -> types::TypeId;
    fn get_type_id_and_value(&self) -> types::TypeIdAndValue;

    fn get_type_id_and_value_required(
        &self,
        required_type: TypeId,
    ) -> Result<TypeIdAndValue, TypeId> {
        let r = self.get_type_id_and_value();
        let tp = r.degrade();
        if tp == required_type {
            Ok(r)
        } else {
            Err(tp)
        }
    }
}

impl<T> ValueTypeMarker for Box<T>
where
    T: ValueTypeMarker + ?Sized,
{
    fn stringify(&self) -> String {
        self.deref().stringify()
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        self.deref().clone_to_value()
    }

    fn get_type_id(&self) -> TypeId {
        self.deref().get_type_id()
    }

    fn get_type_id_and_value(&self) -> types::TypeIdAndValue {
        self.deref().get_type_id_and_value()
    }
}

impl<T> ValueTypeMarker for &mut Box<T>
where
    T: ValueTypeMarker + ?Sized,
{
    fn stringify(&self) -> String {
        self.deref().stringify()
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        self.deref().clone_to_value()
    }

    fn get_type_id(&self) -> TypeId {
        self.deref().get_type_id()
    }

    fn get_type_id_and_value(&self) -> types::TypeIdAndValue {
        self.deref().get_type_id_and_value()
    }
}

pub(crate) struct Value<T>
where
    T: ValueTypeMarker,
{
    base_type_id: TypeId,
    value: T,
}

impl<T> Value<T>
where
    T: ValueTypeMarker,
{
    pub fn new(value: T) -> Self {
        let base_type_id = value.get_type_id();
        Self {
            value,
            base_type_id,
        }
    }

    #[inline]
    pub(crate) fn get_base_type(&self) -> &TypeId {
        &self.base_type_id
    }

    #[inline]
    pub fn get_value(&self) -> &T {
        &self.value
    }
}

impl<T> ValueTypeMarker for Value<T>
where
    T: ValueTypeMarker,
{
    fn stringify(&self) -> String {
        self.value.stringify()
    }

    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        self.value.clone_to_value()
    }

    fn get_type_id(&self) -> TypeId {
        self.value.get_type_id()
    }

    fn get_type_id_and_value(&self) -> TypeIdAndValue {
        self.value.get_type_id_and_value()
    }
}

pub(crate) struct LaterValue<'a> {
    val_expr: &'a Expr,
}

impl<'a> LaterValue<'a> {
    pub(crate) fn new(val_expr: &'a Expr) -> Self {
        Self { val_expr }
    }
    pub(crate) fn compute(&self, frame: &mut EnvFrame) -> Value<Box<dyn ValueTypeMarker>> {
        self.val_expr.eval_in_env(frame)
    }
}

/// A mutable value reference
pub(crate) struct ValRefMut<'a, T>
where
    T: ValueTypeMarker,
{
    reference: &'a mut Value<T>,
}

impl<'a, T> ValRefMut<'a, T>
where
    T: ValueTypeMarker,
{
    pub(crate) fn new(reference: &'a mut Value<T>) -> Self {
        Self { reference }
    }
}

impl<'a, T> ValueTypeMarker for ValRefMut<'a, T>
where
    T: ValueTypeMarker,
{
    #[inline]
    fn stringify(&self) -> String {
        self.reference.stringify()
    }

    #[inline]
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>> {
        // when you clone a reference, it should return a brand new object with the same value
        self.reference.get_value().clone_to_value()
    }

    #[inline]
    fn get_type_id(&self) -> TypeId {
        self.reference.get_value().get_type_id()
    }

    #[inline]
    fn get_type_id_and_value(&self) -> TypeIdAndValue<'_> {
        self.reference.get_value().get_type_id_and_value()
    }
}

pub(crate) fn add_file(file: String, src: String, env: &mut Env) -> usize {
    env.imut.diagnostics_ctx.new_file(file, src)
}

pub(crate) fn interpret<'env>(
    env: &'env mut Env,
    program: &'_ AstProgram,
    file_id: usize,
) -> EnvFrameReturns {
    let statements = program.get_statements();
    let mut frame = EnvFrame {
        variables: HashMap::new(),
        env_frame_data: EnvFrameData::empty(),
        env_ref: &env.imut,
        env_mut_ref: &mut env.mut_,
        file_id,
        fr_type: EnvFrameType::Unknown,
    };

    statements.iter().for_each(|statement| {
        run_in_env_frame(statement, &mut frame);
    });

    let efr = EnvFrameReturns::from(frame.env_frame_data);
    efr.apply_changes_to_env(env);
    efr
}

pub fn start_on(proj_path: &Path, handle: &mut Handle) {
    let path = proj_path.join("build.leaf");
    let path_clone = path.clone();
    let src = String::from_utf8(std::fs::read(path).unwrap()).unwrap() + "\n";
    let src_len = src.len();
    let result = grammar::parse(&src);
    let file_id = add_file(
        path_clone.to_str().unwrap().to_string(),
        src,
        &mut handle.env,
    );
    match result {
        Ok(program) => {
            interpret(&mut handle.env, &program, file_id);
            handle.write_results();
        }
        Err(e) => {
            let syntax_error = match e {
                ParseError::InvalidToken { location } => {
                    SyntaxError::new(location..location + 1, "invalid token")
                }
                ParseError::UnrecognizedEOF { location, expected } => SyntaxError::new(
                    location..location + 1,
                    format!("unrecognized EOF, expected {:?}", expected),
                ),
                ParseError::UnrecognizedToken { token, expected } => SyntaxError::new(
                    token.0..token.2,
                    format!("Unexpected token {}, expected {:?}", token.1, expected),
                ),
                ParseError::ExtraToken { token } => {
                    SyntaxError::new(token.0..token.2, format!("extra token: {}", token.1))
                }
                ParseError::User { error } => match error {
                    LexicalError::UnrecognizedToken { location } => SyntaxError::new(
                        location..location + 1,
                        "Unexpected character at beginning of token",
                    ),
                    LexicalError::StringStartedButNotEnded { start_loc } => {
                        SyntaxError::new(start_loc..src_len, "No end of string found")
                    }
                },
            };
            push_diagnostic_ctx(syntax_error, &handle.env.imut.diagnostics_ctx)
        }
    }
}

pub(crate) struct CallPoolsWrapper {
    global_pool: CallPool,
    num_pool: CallPool,
    bool_pool: CallPool,
    string_pool: CallPool,
    void_pool: CallPool,
    error_pool: CallPool,
    vec_pool: CallPool,
    map_pool: CallPool,
    executable_pool: CallPool,
}

impl CallPoolsWrapper {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            global_pool: get_global_functions(),
            num_pool: types::get_num_call_pool(),
            string_pool: types::get_string_call_pool(),
            bool_pool: types::get_bool_call_pool(),
            void_pool: types::get_void_call_pool(),
            error_pool: types::get_error_call_pool(),
            vec_pool: types::get_vec_call_pool(),
            map_pool: types::get_map_call_pool(),
            executable_pool: types::get_executable_call_pool(),
        }
    }
    #[inline]
    pub(crate) fn get_global_pool(&self) -> &CallPool {
        &self.global_pool
    }

    #[inline]
    pub(crate) fn get_num_pool(&self) -> &CallPool {
        &self.num_pool
    }

    #[inline]
    pub(crate) fn get_bool_pool(&self) -> &CallPool {
        &self.bool_pool
    }

    #[inline]
    pub(crate) fn get_string_pool(&self) -> &CallPool {
        &self.string_pool
    }

    #[inline]
    pub(crate) fn get_void_pool(&self) -> &CallPool {
        &self.void_pool
    }

    #[inline]
    pub(crate) fn get_error_pool(&self) -> &CallPool {
        &self.error_pool
    }

    #[inline]
    pub(crate) fn get_vec_pool(&self) -> &CallPool {
        &self.vec_pool
    }

    #[inline]
    pub(crate) fn get_map_pool(&self) -> &CallPool {
        &self.map_pool
    }

    #[inline]
    pub(crate) fn get_executable_pool(&self) -> &CallPool {
        &self.executable_pool
    }

    #[inline]
    pub(crate) fn get_type_pool(&self, type_: TypeId) -> &CallPool {
        match type_ {
            TypeId::I32 | TypeId::I64 | TypeId::U32 | TypeId::U64 => self.get_num_pool(),
            TypeId::String => self.get_string_pool(),
            TypeId::Void => self.get_void_pool(),
            TypeId::Error => self.get_error_pool(),
            TypeId::Bool => self.get_bool_pool(),
            TypeId::Vec => self.get_vec_pool(),
            TypeId::Map => self.get_map_pool(),
            TypeId::ExecutableReference => self.get_executable_pool(),
        }
    }
}

pub(crate) struct CallPool {
    executors: Vec<CallExecutor>,
}

impl CallPool {
    pub(crate) fn new(executors: Vec<CallExecutor>) -> Self {
        Self { executors }
    }
}

type ExecutorClosure = dyn Fn(
    Location,
    &AstFuncCallArgs,
    &mut EnvFrame,
    Option<&Value<Box<dyn ValueTypeMarker>>>,
) -> Value<Box<dyn ValueTypeMarker>>;

pub(crate) struct CallExecutor {
    name: String,
    aliases: Vec<String>,
    func: Box<ExecutorClosure>,
}

impl CallExecutor {
    pub(crate) fn new<F>(name: String, func: F, aliases: Vec<String>) -> CallExecutor
    where
        F: 'static
            + Fn(
                Location,
                &AstFuncCallArgs,
                &mut EnvFrame,
                Option<&Value<Box<dyn ValueTypeMarker>>>,
            ) -> Value<Box<dyn ValueTypeMarker>>,
    {
        Self {
            name,
            func: Box::new(func),
            aliases,
        }
    }
}

pub(crate) fn func_call_result(
    call: &AstFuncCall,
    frame: &mut EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    eval_call(
        call.get_name(),
        call.get_name_loc(),
        call.get_args(),
        frame,
        frame.get_pools_wrapper().get_global_pool(),
        None,
    )
}

pub(crate) fn method_call_result(
    method_property: &AstPropertyAccess,
    call_args: &AstFuncCallArgs,
    frame: &mut EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    let value = method_property.get_base().eval_in_env(frame);
    eval_call(
        method_property.get_property_name(),
        method_property.get_property_name_loc(),
        call_args,
        frame,
        frame
            .get_pools_wrapper()
            .get_type_pool(value.get_value().get_type_id()),
        Some(&value),
    )
}

pub(crate) fn property_access(
    property: &AstPropertyAccess,
    frame: &mut EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    let base = property.get_base().eval_in_env(frame);
    let property_name = property.get_property_name();
    match base.get_value().get_type_id() {
        TypeId::I32 | TypeId::I64 | TypeId::U32 | TypeId::U64 => {
            resolve_num_property_access(&base, property_name)
        }
        TypeId::String => resolve_str_property_access(base, property_name),
        TypeId::Void => Value::new(Box::new(())),
        TypeId::Error => Value::new(Box::new(types::ErrorValue::new())),
        TypeId::Bool => Value::new(Box::new(())),
        TypeId::Vec => resolve_vec_property_access(base, property_name),
        TypeId::Map => resolve_map_property_access(base, property_name),
        TypeId::ExecutableReference => resolve_executable_property_access(base, property_name),
    }
}

include!("interpreter_internal.rs");
