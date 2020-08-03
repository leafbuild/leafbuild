#[path = "diagnostics/diagnostics.rs"]
pub(crate) mod diagnostics;
pub(crate) mod ops;
pub(crate) mod types;

use crate::interpreter::diagnostics::errors::{IncompatibleAssignmentError, SyntaxError, CannotFindCallError};
use crate::interpreter::diagnostics::{errors, Location};
use crate::{
    grammar::{
        self,
        ast::{
            AstAssignment, AstAtrOp, AstDeclaration, AstFuncCall, AstFuncCallArgs, AstLoc,
            AstProgram, AstPropertyAccess, AstStatement, Expr,
        },
        lexer::LexicalError,
        TokLoc,
    },
    handle::Handle,
    interpreter::{
        diagnostics::{push_diagnostic_ctx, DiagnosticsCtx},
        types::{
            resolve_map_property_access, resolve_num_property_access, resolve_str_property_access,
            resolve_vec_property_access, TypeId, TypeIdAndValue,
        },
    },
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use itertools::Itertools;
use lalrpop_util::ParseError;
use std::{collections::HashMap, ops::Deref, path::Path};

pub struct EnvConfig {
    angry_errors_enabled: bool,

    error_cascade_enabled: bool,
}

impl EnvConfig {
    pub fn new() -> Self {
        Self {
            angry_errors_enabled: false,
            error_cascade_enabled: true,
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
}

pub(crate) struct Env {
    diagnostics_ctx: DiagnosticsCtx,
    call_pools: CallPoolsWrapper,
    config: EnvConfig,
}

impl Env {
    pub(crate) fn new(cfg: EnvConfig) -> Self {
        Self {
            diagnostics_ctx: DiagnosticsCtx::new(
                cfg.angry_errors_enabled,
                cfg.error_cascade_enabled,
            ),
            call_pools: CallPoolsWrapper::new(),
            config: cfg,
        }
    }
    pub(crate) fn modify_config(&mut self) -> &mut EnvConfig {
        &mut self.config
    }
}

pub(crate) enum EnvFrameType {
    Workspace,
    Project,
    Module,
    Unknown, // default value, uninitialized
}

pub(crate) struct EnvFrame<'env> {
    env_ref: &'env Env,
    variables: HashMap<String, Variable<Box<dyn ValueTypeMarker>>>,
    env_frame_returns: EnvFrameReturns,
    file_id: usize,
    fr_type: EnvFrameType,
}

impl<'env> EnvFrame<'env> {
    pub(crate) fn get_value_for_variable(&self, id: &str) -> &Value<Box<dyn ValueTypeMarker>> {
        self.variables
            .iter()
            .find(|&(var_name, _)| var_name == id)
            .unwrap_or_else(|| panic!("No variable named {} found in stack", id))
            .1
            .get_value()
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
}

pub(crate) struct EnvFrameReturns {
    lib_decls: Vec<EnvLibDecl>,
    exe_decls: Vec<EnvExeDecl>,
}

impl EnvFrameReturns {
    pub(crate) fn empty() -> Self {
        Self {
            lib_decls: vec![],
            exe_decls: vec![],
        }
    }
    fn push_to(self, handle: &mut Handle) {
        handle.push_env_frame_returns(self)
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

/// A value reference
pub(crate) struct ValRef<'a, T>
where
    T: ValueTypeMarker,
{
    reference: &'a mut Value<T>,
}

impl<'a, T> ValRef<'a, T>
where
    T: ValueTypeMarker,
{
    pub(crate) fn new(reference: &'a mut Value<T>) -> Self {
        Self { reference }
    }
}

impl<'a, T> ValueTypeMarker for ValRef<'a, T>
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
    env.diagnostics_ctx.new_file(file, src)
}

pub(crate) fn interpret<'env>(
    env: &'env mut Env,
    program: &'_ AstProgram,
    file_id: usize,
) -> EnvFrameReturns {
    let statements = program.get_statements();
    let mut frame = EnvFrame {
        variables: HashMap::new(),
        env_frame_returns: EnvFrameReturns::empty(),
        env_ref: env,
        file_id,
        fr_type: EnvFrameType::Unknown,
    };

    statements.iter().for_each(|statement| {
        run_in_env_frame(statement, &mut frame);
    });

    frame.env_frame_returns
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
            interpret(&mut handle.env, &program, file_id).push_to(handle);
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
            push_diagnostic_ctx(syntax_error, &handle.env.diagnostics_ctx)
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
    pub(crate) fn get_type_pool(&self, type_: TypeId) -> &CallPool {
        match type_ {
            TypeId::I32 | TypeId::I64 | TypeId::U32 | TypeId::U64 => &self.get_num_pool(),
            TypeId::String => &self.get_string_pool(),
            TypeId::Void => &self.get_void_pool(),
            TypeId::Error => &self.get_error_pool(),
            TypeId::Bool => &self.get_bool_pool(),
            TypeId::Vec => &self.get_vec_pool(),
            TypeId::Map => &self.get_map_pool(),
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
    &AstFuncCallArgs,
    &mut EnvFrame,
    Option<&Value<Box<dyn ValueTypeMarker>>>,
) -> Value<Box<dyn ValueTypeMarker>>;

pub(crate) struct CallExecutor {
    name: String,
    func: Box<ExecutorClosure>,
}

impl CallExecutor {
    pub(crate) fn new<F>(name: String, func: F) -> CallExecutor
    where
        F: 'static
            + Fn(
                &AstFuncCallArgs,
                &mut EnvFrame,
                Option<&Value<Box<dyn ValueTypeMarker>>>,
            ) -> Value<Box<dyn ValueTypeMarker>>,
    {
        Self {
            name,
            func: Box::new(func),
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
        TypeId::String => resolve_str_property_access(&base, property_name),
        TypeId::Void => Value::new(Box::new(())),
        TypeId::Error => Value::new(Box::new(types::ErrorValue::new())),
        TypeId::Bool => Value::new(Box::new(())),
        TypeId::Vec => resolve_vec_property_access(&base, property_name),
        TypeId::Map => resolve_map_property_access(&base, property_name),
    }
}

include!("interpreter_internal.rs");
