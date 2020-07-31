#[path = "errors/errors.rs"]
pub(crate) mod errors;
pub(crate) mod ops;
mod types;

use crate::grammar::ast::AstDeclaration;
use crate::{
    grammar::{
        self,
        ast::{
            AstAssignment, AstAtrOp, AstFuncCall, AstFuncCallArgs, AstLoc, AstProgram,
            AstPropertyAccess, AstStatement,
        },
        TokLoc,
    },
    handle::Handle,
    interpreter::errors::ErrCtx,
    interpreter::types::TypeIdAndValue,
    interpreter::types::{resolve_num_property_access, resolve_str_property_access, TypeId},
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use itertools::Itertools;
use std::ops::Range;
use std::{collections::HashMap, ops::Deref, path::Path};

pub(crate) struct Env {
    errctx: ErrCtx,
    call_pools: CallPoolsWrapper,
    #[cfg(feature = "angry-errors")]
    angry_errors: bool,
}

impl Env {
    pub(crate) fn new() -> Self {
        Self {
            errctx: ErrCtx::new(),
            call_pools: CallPoolsWrapper::new(),
            #[cfg(feature = "angry-errors")]
            angry_errors: false,
        }
    }

    #[cfg(feature = "angry-errors")]
    pub(crate) fn set_angry_errors_mode(&mut self, enabled: bool) {
        self.angry_errors = enabled;
    }
}

pub(crate) struct EnvFrame<'env> {
    env_ref: &'env Env,
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

    #[inline]
    pub(crate) fn get_errctx(&self) -> &ErrCtx {
        &self.env_ref.errctx
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
    pub(crate) fn get_value(&self) -> &Value<T> {
        &self.value
    }
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

    #[inline]
    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    #[inline]
    pub fn stringify(&self) -> String {
        self.value.stringify()
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

pub(crate) struct TakeRefError {
    diagnostic: Diagnostic<usize>,
}

impl TakeRefError {
    pub(crate) fn new(diagnostic: Diagnostic<usize>) -> Self {
        Self { diagnostic }
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
    let path_clone = path.clone();
    let src = String::from_utf8(std::fs::read(path).unwrap()).unwrap() + "\n";
    let program = grammar::parse(&src).unwrap();
    interpret(
        &mut handle.env,
        &program,
        path_clone.to_str().unwrap().to_string(),
        src,
    )
    .push_to(handle);
}

pub(crate) struct CallPoolsWrapper {
    global_pool: CallPool,
    num_pool: CallPool,
    string_pool: CallPool,
}

impl CallPoolsWrapper {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            global_pool: get_global_functions(),
            num_pool: types::get_num_call_pool(),
            string_pool: types::get_string_call_pool(),
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
    pub(crate) fn get_string_pool(&self) -> &CallPool {
        &self.string_pool
    }

    #[inline]
    pub(crate) fn get_type_pool(&self, type_: TypeId) -> &CallPool {
        match type_ {
            TypeId::I32 | TypeId::I64 | TypeId::U32 | TypeId::U64 => &self.get_num_pool(),
            TypeId::String => &self.get_string_pool(),
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
        types::TypeId::I32 | types::TypeId::I64 | types::TypeId::U32 | types::TypeId::U64 => {
            resolve_num_property_access(&base, property_name)
        }
        types::TypeId::String => resolve_str_property_access(&base, property_name),
    }
}

include!("interpreter_internal.rs");
