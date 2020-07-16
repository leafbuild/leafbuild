mod ops;
mod types;

use libcmbscore::utils::Stack;
use std::collections::HashMap;
use crate::grammar::ast::{AstProgram, AstStatement, AstFuncCall, AstFuncCallArgs};
use crate::grammar::ast::Expr::FuncCall;
use crate::handle::Handle;


struct Env {
    frames: Stack<EnvFrame>,
}

pub(crate) struct EnvFrame {
    variables: HashMap<String, Variable<Box<dyn ValueTypeMarker>>>,
    env_frame_returns: EnvFrameReturns,
}

pub(crate) struct EnvFrameReturns {
    lib_decls: Vec<EnvLibDecl>,
    exe_decls: Vec<EnvExeDecl>,
}

impl EnvFrameReturns {
    pub(crate) fn empty() -> Self { Self { lib_decls: vec![], exe_decls: vec![] } }
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

struct Variable<T> where T: ValueTypeMarker + Sized {
    name: String,
    value: Value<T>,
}

pub(crate) trait ValueTypeMarker {}

impl<T: ?Sized> ValueTypeMarker for Box<T> where T: ValueTypeMarker {}

pub(crate) struct Value<T> where T: ValueTypeMarker + Sized {
    value: T,
}

impl<T> Value<T> where T: ValueTypeMarker {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }
    pub fn get_value(&self) -> &T { &self.value }
}

pub(crate) fn interpret(program: &AstProgram, handle: &mut Handle) -> EnvFrameReturns {
    let statements = program.get_statements();
    let mut frame = EnvFrame {
        variables: HashMap::new(),
        env_frame_returns: EnvFrameReturns::empty(),
    };

    statements.iter().for_each(|statement| {
        run_in_env_frame(handle, statement, &mut frame);
    });

    frame.env_frame_returns
}

pub fn interpret_wrapper(program: &AstProgram, handle: &mut Handle) {
    interpret(program, handle).push_to(handle);
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
        where F: 'static + Fn(&AstFuncCallArgs, &mut EnvFrame) -> Value<Box<dyn ValueTypeMarker>> {
        Self {
            name,
            func: Box::new(func),
        }
    }
}

include!("interpreter_internal.rs");