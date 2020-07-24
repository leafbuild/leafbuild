use crate::grammar::lexer::TokLoc;
use crate::{
    interpreter,
    interpreter::{EnvFrame, Value, ValueTypeMarker},
};

pub enum Atom {
    Number((i32, TokLoc)),
    Str((String, TokLoc)),
    Id((String, TokLoc)),
}

pub enum Expr {
    Atom(Atom),
    Op(Box<Expr>, Opcode, Box<Expr>),
    FuncCall(AstFuncCall),
    MethodCall(AstMethodCall),
    PropertyAccess(AstPropertyAccess),
}

pub struct AstPropertyAccess {
    base: Box<Expr>,
    property_name: (String, TokLoc),
}

impl AstPropertyAccess {
    pub fn new(base: Box<Expr>, property_name: (String, TokLoc)) -> Self {
        Self {
            base,
            property_name,
        }
    }

    #[inline]
    pub(crate) fn get_base(&self) -> &Expr {
        &self.base
    }

    #[inline]
    pub(crate) fn get_property_name(&self) -> &String {
        &self.property_name.0
    }
}

impl Expr {
    pub(crate) fn eval_in_env(&self, frame: &mut EnvFrame) -> Value<Box<dyn ValueTypeMarker>> {
        match self {
            Expr::Atom(Atom::Number((num, _loc))) => Value::new(Box::new(*num)),
            Expr::Atom(Atom::Str((str, _loc))) => Value::new(Box::new(str.clone())),
            Expr::Atom(Atom::Id((id, _loc))) => frame
                .get_value_for_variable(&id.clone())
                .get_value()
                .clone_to_value(),
            Expr::FuncCall(call_expr) => interpreter::func_call_result(call_expr, frame),
            Expr::MethodCall(call_expr) => {
                interpreter::method_call_result(&call_expr.method_property, &call_expr.args, frame)
            }
            Expr::Op(left, opcode, right) => {
                let ls = left.eval_in_env(frame);
                let rs = right.eval_in_env(frame);
                opcode.compute_result_for(ls, rs)
            } // will implement later
            Expr::PropertyAccess(access) => interpreter::property_access(access, frame),
        }
    }
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Mod,
}

impl Opcode {
    pub(crate) fn compute_result_for(
        &self,
        ls: Value<Box<dyn ValueTypeMarker>>,
        rs: Value<Box<dyn ValueTypeMarker>>,
    ) -> Value<Box<dyn ValueTypeMarker>> {
        match self {
            Opcode::Add => interpreter::ops::op_add(ls, rs),
            Opcode::Sub => interpreter::ops::op_sub(ls, rs),
            Opcode::Mul => interpreter::ops::op_mul(ls, rs),
            Opcode::Div => interpreter::ops::op_div(ls, rs),
            Opcode::Mod => interpreter::ops::op_mod(ls, rs),
        }
    }
}

pub struct AstFuncCall {
    func_name: (String, TokLoc),
    func_args: AstFuncCallArgs,
}

impl AstFuncCall {
    pub fn new(name: (String, TokLoc), call_args: AstFuncCallArgs) -> AstFuncCall {
        AstFuncCall {
            func_name: name,
            func_args: call_args,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.func_name.0
    }

    pub fn get_args(&self) -> &AstFuncCallArgs {
        &self.func_args
    }
}

pub struct AstFuncCallArgs {
    positional_args: Vec<AstPositionalArg>,
    named_args: Vec<AstNamedArg>,
}

impl AstFuncCallArgs {
    pub fn new(
        positional_args: Vec<AstPositionalArg>,
        named_args: Vec<AstNamedArg>,
    ) -> AstFuncCallArgs {
        AstFuncCallArgs {
            positional_args,
            named_args,
        }
    }

    pub fn new_only_positional(positional_args: Vec<AstPositionalArg>) -> AstFuncCallArgs {
        Self::new(positional_args, vec![])
    }

    pub fn new_only_named(named_args: Vec<AstNamedArg>) -> AstFuncCallArgs {
        Self::new(vec![], named_args)
    }

    pub fn empty() -> AstFuncCallArgs {
        Self::new(vec![], vec![])
    }

    pub fn get_positional_args(&self) -> &Vec<AstPositionalArg> {
        &self.positional_args
    }
    pub fn get_named_args(&self) -> &Vec<AstNamedArg> {
        &self.named_args
    }
}

pub struct AstPositionalArg {
    value: Expr,
}

impl AstPositionalArg {
    pub(crate) fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl From<Box<Expr>> for AstPositionalArg {
    fn from(b: Box<Expr>) -> Self {
        Self { value: *b }
    }
}

pub struct AstNamedArg {
    name: (String, TokLoc),
    value: Box<Expr>,
}

impl AstNamedArg {
    pub fn get_name(&self) -> &String {
        &self.name.0
    }
    pub fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl From<((String, TokLoc), Box<Expr>)> for AstNamedArg {
    fn from(v: ((String, TokLoc), Box<Expr>)) -> Self {
        let (name, value) = v;
        Self { name, value }
    }
}

pub struct AstMethodCall {
    method_property: AstPropertyAccess,
    args: AstFuncCallArgs,
}

impl AstMethodCall {
    pub fn new(method_property: AstPropertyAccess, args: AstFuncCallArgs) -> AstMethodCall {
        AstMethodCall {
            method_property,
            args,
        }
    }

    pub(crate) fn get_base_expr(&self) -> &Expr {
        &self.method_property.base
    }

    pub(crate) fn get_name(&self) -> &String {
        &self.method_property.get_property_name()
    }
    pub(crate) fn get_args(&self) -> &AstFuncCallArgs {
        &self.args
    }
}

pub struct AstAssignment {
    name: (String, TokLoc),
    op: AstAtrOp,
    value: Box<Expr>,
}

impl AstAssignment {
    pub fn new(name: (String, TokLoc), op: AstAtrOp, value: Box<Expr>) -> AstAssignment {
        AstAssignment { name, op, value }
    }

    #[inline]
    pub fn get_name(&self) -> &String {
        &self.name.0
    }

    #[inline]
    pub fn get_op(&self) -> &AstAtrOp {
        &self.op
    }

    #[inline]
    pub fn get_value(&self) -> &Expr {
        &self.value
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

pub struct AstProgram {
    statements: Vec<AstStatement>,
}

impl AstProgram {
    pub fn new(statements: Vec<AstStatement>) -> AstProgram {
        AstProgram { statements }
    }
    pub fn get_statements(&self) -> &Vec<AstStatement> {
        &self.statements
    }
}
