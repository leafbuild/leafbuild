#[path = "errors/errors.rs"]
pub(crate) mod errors;
pub(crate) mod ops;
mod types;

use crate::grammar::ast::{AstAssignment, AstPropertyAccess};
use crate::grammar::TokLoc;
use crate::interpreter::errors::ErrCtx;
use crate::interpreter::types::{resolve_num_property_access, resolve_str_property_access, TypeId};
use crate::{
    grammar,
    grammar::ast::{AstAtrOp, AstFuncCall, AstFuncCallArgs, AstProgram, AstStatement},
    handle::Handle,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use itertools::Itertools;
use libleafcore::utils::Stack;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};

pub(crate) struct Env<'env> {
    frames: Stack<EnvFrame<'env>>,
    errctx: ErrCtx,
}

impl<'env> Env<'env> {
    pub(crate) fn new() -> Self {
        Self {
            frames: Stack::new(),
            errctx: ErrCtx::new(),
        }
    }
}

pub(crate) struct EnvFrame<'env> {
    env_ref: &'env Env<'env>,
    variables: HashMap<String, Variable<Box<dyn ValueTypeMarker>>>,
    env_frame_returns: EnvFrameReturns,
    file_id: usize,
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

    pub(crate) fn get_errctx(&self) -> &ErrCtx {
        &self.env_ref.errctx
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

struct Variable<T>
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
    pub(crate) fn get_value(&self) -> &Value<T> {
        &self.value
    }
}

pub(crate) trait ValueTypeMarker {
    fn stringify(&self) -> String;
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>>;
    fn get_type_id(&self) -> types::TypeId;
    fn get_type_id_and_value(&self) -> types::TypeIdAndValue;
    fn get_func_call_pool(&self) -> CallPool {
        get_func_call_pool_for_typeid(self.get_type_id())
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

pub(crate) struct Value<T>
where
    T: ValueTypeMarker,
{
    value: T,
}

impl<T> Value<T>
where
    T: ValueTypeMarker,
{
    pub fn new(value: T) -> Self {
        Self { value }
    }
    #[inline]
    pub fn get_value(&self) -> &T {
        &self.value
    }
    pub fn stringify(&self) -> String {
        self.value.stringify()
    }
}

pub(crate) fn interpret<'env>(
    env: &'env mut Env,
    program: &'_ AstProgram,
    file: String,
    src: String,
) -> EnvFrameReturns {
    let statements = program.get_statements();
    let file_id = env.errctx.new_file(file, src);
    let mut frame = EnvFrame {
        variables: HashMap::new(),
        env_frame_returns: EnvFrameReturns::empty(),
        env_ref: env,
        file_id,
    };

    statements.iter().for_each(|statement| {
        run_in_env_frame(statement, &mut frame);
    });

    frame.env_frame_returns
}

pub fn start_on(proj_path: &Path, handle: &mut Handle) {
    let path = proj_path.join("build.leaf");
    let src = String::from_utf8(std::fs::read(path).unwrap()).unwrap() + "\n";
    let program = grammar::parse(&src).unwrap();
    interpret(
        &mut handle.env,
        &program,
        proj_path.to_str().unwrap().to_string(),
        src,
    )
    .push_to(handle);
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
        &get_global_functions(),
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
        &value.get_value().get_func_call_pool(),
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
        types::TypeId::I32 | types::TypeId::I64 | types::TypeId::U32 | types::TypeId::U64 => {
            resolve_num_property_access(&base, property_name)
        }
        types::TypeId::String => resolve_str_property_access(&base, property_name),
    }
}

include!("interpreter_internal.rs");
