use crate::grammar::{
    lexer::Tok,
};

pub enum Atom {
    Number(i32),
    Str(String),
    Id(String),
}

pub enum Expr {
    Atom(Atom),
    Op(Box<Expr>, Opcode, Box<Expr>),
    FuncCall(AstFuncCall),
    MethodCall(AstMethodCall),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Mod,
}

pub struct AstFuncCall {
    func_name: String,
    func_args: AstFuncCallArgs,
}

impl AstFuncCall {
    pub fn new(name: String, call_args: AstFuncCallArgs) -> AstFuncCall {
        AstFuncCall {
            func_name: name,
            func_args: call_args,
        }
    }

    pub fn get_name(&self) -> &String { &self.func_name }

    pub fn get_args(&self) -> &AstFuncCallArgs { &self.func_args }
}

pub struct AstFuncCallArgs {
    positional_args: Vec<AstPositionalArg>,
    named_args: Vec<AstNamedArg>,
}

impl AstFuncCallArgs {
    pub fn new(positional_args: Vec<AstPositionalArg>, named_args: Vec<AstNamedArg>) -> AstFuncCallArgs {
        AstFuncCallArgs {
            positional_args,
            named_args,
        }
    }

    pub fn new_only_positional(positional_args: Vec<AstPositionalArg>) -> AstFuncCallArgs { Self::new(positional_args, vec![]) }

    pub fn new_only_named(named_args: Vec<AstNamedArg>) -> AstFuncCallArgs { Self::new(vec![], named_args) }

    pub fn empty() -> AstFuncCallArgs { Self::new(vec![], vec![]) }

    pub fn get_positional_args(&self) -> &Vec<AstPositionalArg> { &self.positional_args }
    pub fn get_named_args(&self) -> &Vec<AstNamedArg> { &self.named_args }
}

pub struct AstPositionalArg {
    value: Expr,
}

impl From<Box<Expr>> for AstPositionalArg {
    fn from(b: Box<Expr>) -> Self {
        Self {
            value: *b
        }
    }
}

pub struct AstNamedArg {
    name: String,
    value: Box<Expr>,
}

impl AstNamedArg {
    pub fn get_name(&self) -> &String { &self.name }
}

impl From<(String, Box<Expr>)> for AstNamedArg {
    fn from(v: (String, Box<Expr>)) -> Self {
        let (name, value) = v;
        Self {
            name,
            value,
        }
    }
}

pub struct AstMethodCall {
    base: Box<Expr>,
    call: AstFuncCall,
}

impl AstMethodCall {
    pub fn new(base: Box<Expr>, call: AstFuncCall) -> AstMethodCall {
        AstMethodCall {
            base,
            call,
        }
    }
}

pub struct AstAssignment {
    name: String,
    op: AstAtrOp,
    value: Box<Expr>,
}

impl AstAssignment {
    pub fn new(name: String, op: AstAtrOp, value: Box<Expr>) -> AstAssignment {
        AstAssignment {
            name,
            op,
            value,
        }
    }
}

pub enum AstAtrOp {
    Atr,
    AddAtr,
    SubAtr,
    MulAtr,
    DivAtr,
    ModAtr,
}

pub enum AstStatement {
    FuncCall(AstFuncCall),
    MethodCall(AstMethodCall),
    Assignment(AstAssignment),
}