mod ops;
mod types;

use crate::{
    grammar::ast::{AstFuncCall, AstFuncCallArgs, AstProgram, AstStatement, Expr},
    handle::Handle,
};
use itertools::Itertools;
use libcmbscore::utils::Stack;
use std::collections::HashMap;
use std::ops::Deref;

struct Env {
    frames: Stack<EnvFrame>,
}

pub(crate) struct EnvFrame {
    variables: HashMap<String, Variable<Box<dyn ValueTypeMarker>>>,
    env_frame_returns: EnvFrameReturns,
}

impl EnvFrame {
    pub(crate) fn get_value_for_variable(&self, id: &String) -> &Value<Box<dyn ValueTypeMarker>> {
        self.variables
            .iter()
            .find(|&(var_name, _)| var_name == id)
            .unwrap_or_else(|| panic!("No variable named {} found in stack", id))
            .1
            .get_value()
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
    pub(crate) fn get_value(&self) -> &Value<T> {
        &self.value
    }
}

pub(crate) trait ValueTypeMarker {
    fn stringify(&self) -> String;
    fn clone_to_value(&self) -> Value<Box<dyn ValueTypeMarker>>;
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
    pub fn get_value(&self) -> &T {
        &self.value
    }
    pub fn stringify(&self) -> String {
        self.value.stringify()
    }
}

pub(crate) fn interpret(program: &AstProgram) -> EnvFrameReturns {
    let statements = program.get_statements();
    let mut frame = EnvFrame {
        variables: HashMap::new(),
        env_frame_returns: EnvFrameReturns::empty(),
    };

    statements.iter().for_each(|statement| {
        run_in_env_frame(statement, &mut frame);
    });

    frame.env_frame_returns
}

pub fn interpret_wrapper(program: &AstProgram, handle: &mut Handle) {
    interpret(program).push_to(handle);
}

pub(crate) struct FuncCallPool {
    executors: Vec<FuncCallExecutor>,
}

impl FuncCallPool {
    pub(crate) fn new(executors: Vec<FuncCallExecutor>) -> Self {
        Self { executors }
    }
}

pub(crate) struct FuncCallExecutor {
    name: String,
    func: Box<dyn Fn(&AstFuncCallArgs, &mut EnvFrame) -> Value<Box<dyn ValueTypeMarker>>>,
}

impl FuncCallExecutor {
    pub(crate) fn new<F>(name: String, func: F) -> FuncCallExecutor
    where
        F: 'static + Fn(&AstFuncCallArgs, &mut EnvFrame) -> Value<Box<dyn ValueTypeMarker>>,
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
    eval_func_call(call, frame, &get_global_functions())
}

pub(crate) fn method_call_result(
    base: &Expr,
    call: &AstFuncCall,
    frame: &mut EnvFrame,
) -> Value<Box<dyn ValueTypeMarker>> {
    Value::new(Box::new(0))
}

include!("interpreter_internal.rs");
