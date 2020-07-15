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

pub(crate) fn interpret(program: &AstProgram) -> EnvFrameReturns {
    let statements = program.get_statements();
    EnvFrameReturns {
        lib_decls: vec![],
        exe_decls: vec![],
    }
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
    func: Box<dyn Fn(AstFuncCallArgs, &mut EnvFrame) -> Value<Box<dyn ValueTypeMarker>>>,
}

impl FuncCallExecutor {
    pub(crate) fn new<F>(func: F) -> FuncCallExecutor
        where F: 'static + Fn(AstFuncCallArgs, &mut EnvFrame) -> Value<Box<dyn ValueTypeMarker>> {
        Self {
            func: Box::new(func)
        }
    }
}

include!("interpreter_internal.rs");