mod ops;

use libcmbscore::utils::Stack;
use std::collections::HashMap;
use crate::grammar::ast::AstProgram;

struct Env {
    frames: Stack<EnvFrame>,
}

struct EnvFrame {
    variables: HashMap<String, Variable<Box<dyn ValType>>>,
    env_frame_returns: Vec<EnvFrameReturn>,
}

struct EnvFrameReturn {
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

struct Variable<T> where T: ValType {
    name: String,
    value: Value<T>,
}

pub trait ValType {}

impl ValType for Box<dyn ValType> {}

pub struct Value<T> where T: ValType + Sized {
    value: T,
}

impl<T> Value<T> where T: ValType + Sized {
    pub fn get_value(&self) -> &T { &self.value }
}

fn interpret(program: &AstProgram) -> EnvFrameReturn {
    let statements = program.get_statements();
    EnvFrameReturn {
        lib_decls: vec![],
        exe_decls: vec![],
    }
}