use crate::interpreter::errors::Location;
use crate::interpreter::ops::{OpsError, OpsErrorType};
use crate::interpreter::{errors, LaterValue};
use crate::{
    grammar::lexer::TokLoc,
    interpreter::{self, EnvFrame, TakeRefError, ValRef, Value, ValueTypeMarker},
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::collections::HashMap;

pub(crate) trait AstLoc {
    fn get_begin(&self) -> usize;

    fn get_end(&self) -> usize;

    fn get_rng(&self) -> errors::Location;
}

pub enum Atom {
    Number((i32, TokLoc)),
    Bool((bool, TokLoc)),
    Str((String, TokLoc)),
    Id((String, TokLoc)),
    #[allow(clippy::vec_box)] // needed by the way the grammar works
    ArrayLit((Vec<Box<Expr>>, TokLoc)),
    MapLit((Vec<AstNamedExpr>, TokLoc)),
}

impl AstLoc for Atom {
    #[inline]
    fn get_begin(&self) -> usize {
        match self {
            Atom::Bool((_, loc))
            | Atom::Number((_, loc))
            | Atom::Id((_, loc))
            | Atom::Str((_, loc))
            | Atom::ArrayLit((_, loc))
            | Atom::MapLit((_, loc)) => loc.get_begin(),
        }
    }

    #[inline]
    fn get_end(&self) -> usize {
        match self {
            Atom::Bool((_, loc))
            | Atom::Number((_, loc))
            | Atom::Id((_, loc))
            | Atom::Str((_, loc))
            | Atom::ArrayLit((_, loc))
            | Atom::MapLit((_, loc)) => loc.get_end(),
        }
    }

    fn get_rng(&self) -> errors::Location {
        match self {
            Atom::Bool((_, loc))
            | Atom::Number((_, loc))
            | Atom::Id((_, loc))
            | Atom::Str((_, loc))
            | Atom::ArrayLit((_, loc))
            | Atom::MapLit((_, loc)) => loc.as_rng(),
        }
    }
}

pub enum Expr {
    Atom(Atom),
    Op(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(UnaryOpcode, Box<Expr>),
    FuncCall(AstFuncCall),
    MethodCall(AstMethodCall),
    PropertyAccess(AstPropertyAccess),
    ParenExpr(usize, Box<Expr>, usize),
}

impl Expr {
    pub(crate) fn eval_in_env(&self, frame: &mut EnvFrame) -> Value<Box<dyn ValueTypeMarker>> {
        match self {
            Expr::Atom(Atom::Number((num, _loc))) => Value::new(Box::new(*num)),
            Expr::Atom(Atom::Bool((v, _loc))) => Value::new(Box::new(*v)),
            Expr::Atom(Atom::Str((str, _loc))) => Value::new(Box::new(str.clone())),
            Expr::Atom(Atom::Id((id, _loc))) => frame
                .get_value_for_variable(&id.clone())
                .get_value()
                .clone_to_value(),
            Expr::Atom(Atom::ArrayLit((v, _loc))) => {
                let mut val = Vec::with_capacity(v.capacity());
                v.iter().for_each(|x| val.push(x.eval_in_env(frame)));
                Value::new(Box::new(val))
            }
            Expr::Atom(Atom::MapLit((v, _loc))) => {
                let mut val = HashMap::with_capacity(v.capacity());
                v.iter().for_each(|x: &AstNamedExpr| {
                    val.insert(x.name.0.clone(), x.value.eval_in_env(frame));
                });
                Value::new(Box::new(val))
            }
            Expr::FuncCall(call_expr) => interpreter::func_call_result(call_expr, frame),
            Expr::MethodCall(call_expr) => {
                interpreter::method_call_result(&call_expr.method_property, &call_expr.args, frame)
            }
            Expr::Op(left, opcode, right) => {
                let v = opcode.compute_result_for(left, right, frame);
                match v {
                    Ok(v) => v,
                    Err(err) => {
                        let should_print_err = match err.get_type() {
                            OpsErrorType::Incompatible => true,
                            OpsErrorType::IncompatibleError => {
                                frame.get_errctx().get_error_cascade()
                            }
                        };
                        if should_print_err {
                            errors::push_diagnostic(frame, err.get_diagnostic());
                        }
                        Value::new(Box::new(interpreter::types::ErrorValue::new()))
                    }
                }
            }
            Expr::UnaryOp(opcode, right) => {
                let v = opcode.compute_result_for(right, frame);
                match v {
                    Ok(v) => v,
                    Err(err) => {
                        let should_print_err = match err.get_type() {
                            OpsErrorType::Incompatible => true,
                            OpsErrorType::IncompatibleError => {
                                frame.get_errctx().get_error_cascade()
                            }
                        };
                        if should_print_err {
                            errors::push_diagnostic(frame, err.get_diagnostic());
                        }
                        Value::new(Box::new(interpreter::types::ErrorValue::new()))
                    }
                }
            }
            Expr::PropertyAccess(access) => interpreter::property_access(access, frame),
            Expr::ParenExpr(_, e, _) => e.eval_in_env(frame),
        }
    }

    pub(crate) fn eval_ref<'a>(
        &self,
        frame: &'a mut EnvFrame,
    ) -> Result<ValRef<'a, Box<dyn ValueTypeMarker>>, TakeRefError> {
        match self {
            Expr::Atom(atom) => match atom {
                Atom::Id((name, _)) => Ok(ValRef::new(
                    frame
                        .get_variables_mut()
                        .get_mut(name)
                        .unwrap()
                        .get_value_mut(),
                )),
                Atom::Number((_, loc)) => Err(TakeRefError::new(
                    Diagnostic::error()
                        .with_message(errors::get_error(
                            "cannot take a reference from a non-id",
                            frame,
                        ))
                        .with_labels(vec![Label::primary(frame.get_file_id(), loc.as_rng())
                            .with_message(errors::get_error(
                                "cannot take a reference from a number literal",
                                frame,
                            ))]),
                )),
                Atom::Str((_, loc)) => Err(TakeRefError::new(
                    Diagnostic::error()
                        .with_message(errors::get_error(
                            "cannot take a reference from a non-id",
                            frame,
                        ))
                        .with_labels(vec![Label::primary(frame.get_file_id(), loc.as_rng())
                            .with_message(errors::get_error(
                                "cannot take a reference from a string",
                                frame,
                            ))]),
                )),
                Atom::Bool((_v, loc)) => Err(TakeRefError::new(
                    Diagnostic::error()
                        .with_message(errors::get_error(
                            "cannot take a reference from a non-id",
                            frame,
                        ))
                        .with_labels(vec![Label::primary(frame.get_file_id(), loc.as_rng())
                            .with_message(errors::get_error(
                                "cannot take a reference from a bool",
                                frame,
                            ))]),
                )),
                Atom::ArrayLit((_v, loc)) => Err(TakeRefError::new(
                    Diagnostic::error()
                        .with_message(errors::get_error(
                            "cannot take a reference from a non-id",
                            frame,
                        ))
                        .with_labels(vec![Label::primary(frame.get_file_id(), loc.as_rng())
                            .with_message(errors::get_error(
                                "cannot take a reference from an array literal",
                                frame,
                            ))]),
                )),
                Atom::MapLit((_v, loc)) => Err(TakeRefError::new(
                    Diagnostic::error()
                        .with_message(errors::get_error(
                            "cannot take a reference from a non-id",
                            frame,
                        ))
                        .with_labels(vec![Label::primary(frame.get_file_id(), loc.as_rng())
                            .with_message(errors::get_error(
                                "cannot take a reference from a map literal",
                                frame,
                            ))]),
                )),
            },
            Expr::FuncCall(_) => Err(TakeRefError::new(
                Diagnostic::error()
                    .with_message(errors::get_error(
                        "cannot take a reference from a non-id",
                        frame,
                    ))
                    .with_labels(vec![Label::primary(frame.get_file_id(), self.get_rng())
                        .with_message(errors::get_error(
                            "cannot take a reference from a function call",
                            frame,
                        ))]),
            )),
            Expr::MethodCall(_) => Err(TakeRefError::new(
                Diagnostic::error()
                    .with_message(errors::get_error(
                        "cannot take a reference from a non-id",
                        frame,
                    ))
                    .with_labels(vec![Label::primary(frame.get_file_id(), self.get_rng())
                        .with_message(errors::get_error(
                            "cannot take a reference from a method call",
                            frame,
                        ))]),
            )),
            Expr::PropertyAccess(_) => Err(TakeRefError::new(
                Diagnostic::error()
                    .with_message(errors::get_error(
                        "cannot take a reference from a non-id",
                        frame,
                    ))
                    .with_labels(vec![Label::primary(frame.get_file_id(), self.get_rng())
                        .with_message(errors::get_error(
                            "cannot take a reference from a property access",
                            frame,
                        ))]),
            )),
            Expr::Op(_, _, _) => Err(TakeRefError::new(
                Diagnostic::error()
                    .with_message(errors::get_error(
                        "cannot take a reference from a non-id",
                        frame,
                    ))
                    .with_labels(vec![Label::primary(frame.get_file_id(), self.get_rng())
                        .with_message(errors::get_error(
                            "cannot take a reference from an non-id expression",
                            frame,
                        ))]),
            )),
            Expr::UnaryOp(_, _) => Err(TakeRefError::new(
                Diagnostic::error()
                    .with_message(errors::get_error(
                        "cannot take a reference from a non-id",
                        frame,
                    ))
                    .with_labels(vec![Label::primary(frame.get_file_id(), self.get_rng())
                        .with_message(errors::get_error(
                            "cannot take a reference from an non-id expression",
                            frame,
                        ))]),
            )),
            Expr::ParenExpr(_, e, _) => e.eval_ref(frame),
        }
    }

    fn later_eval(&self) -> LaterValue {
        LaterValue::new(self)
    }
}

impl AstLoc for Expr {
    #[inline]
    fn get_begin(&self) -> usize {
        match self {
            Expr::Atom(atm) => atm.get_begin(),
            Expr::FuncCall(call) => call.get_begin(),
            Expr::MethodCall(call) => call.get_begin(),
            Expr::Op(expr, _, _) => expr.get_begin(),
            Expr::PropertyAccess(prop_access) => prop_access.get_begin(),
            Expr::ParenExpr(begin, _, _) => *begin,
            Expr::UnaryOp(op, _) => op.get_begin(),
        }
    }

    #[inline]
    fn get_end(&self) -> usize {
        match self {
            Expr::Atom(atm) => atm.get_end(),
            Expr::FuncCall(call) => call.get_end(),
            Expr::MethodCall(call) => call.get_end(),
            Expr::Op(expr, _, _) => expr.get_end(),
            Expr::PropertyAccess(prop_access) => prop_access.get_end(),
            Expr::ParenExpr(_, _, end) => *end,
            Expr::UnaryOp(_, expr) => expr.get_end(),
        }
    }

    #[inline]
    fn get_rng(&self) -> errors::Location {
        match self {
            Expr::Atom(atm) => atm.get_rng(),
            Expr::FuncCall(call) => call.get_rng(),
            Expr::MethodCall(call) => call.get_rng(),
            Expr::Op(expr1, _, expr2) => expr1.get_begin()..expr2.get_end(),
            Expr::UnaryOp(op, expr) => op.get_begin()..expr.get_end(),
            Expr::PropertyAccess(prop_access) => prop_access.get_rng(),
            Expr::ParenExpr(begin, _, end) => *begin..*end,
        }
    }
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

    #[inline]
    pub(crate) fn get_property_name_loc(&self) -> &TokLoc {
        &self.property_name.1
    }
}

impl AstLoc for AstPropertyAccess {
    #[inline]
    fn get_begin(&self) -> usize {
        self.base.get_begin()
    }

    #[inline]
    fn get_end(&self) -> usize {
        self.property_name.1.get_end()
    }

    #[inline]
    fn get_rng(&self) -> errors::Location {
        self.get_begin()..self.get_end()
    }
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Mod,
    And,
    Or,
    In,
    NotIn,
    Equal,
    G,
    L,
    GE,
    LE,
    NE,
}

impl Opcode {
    pub(crate) fn compute_result_for(
        &self,
        ls: &Expr,
        rs: &Expr,
        frame: &mut EnvFrame,
    ) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
        match self {
            Opcode::Add => interpreter::ops::op_add(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::Sub => interpreter::ops::op_sub(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::Mul => interpreter::ops::op_mul(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::Div => interpreter::ops::op_div(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::Mod => interpreter::ops::op_mod(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::And => interpreter::ops::op_and(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.later_eval(),
                rs.get_rng(),
                frame,
                frame.get_file_id(),
            ),
            Opcode::Or => interpreter::ops::op_or(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.later_eval(),
                rs.get_rng(),
                frame,
                frame.get_file_id(),
            ),
            Opcode::In => Ok(Value::new(Box::new(()))),
            Opcode::NotIn => Ok(Value::new(Box::new(()))),
            Opcode::Equal => interpreter::ops::op_eq(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::G => interpreter::ops::op_g(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::L => interpreter::ops::op_l(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::GE => interpreter::ops::op_ge(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::LE => interpreter::ops::op_le(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
            Opcode::NE => interpreter::ops::op_neq(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_file_id(),
            ),
        }
    }
}

pub enum UnaryOpcode {
    Not(TokLoc),
    Neg(TokLoc),
}

impl UnaryOpcode {
    pub(crate) fn compute_result_for(
        &self,
        expr: &Expr,
        frame: &mut EnvFrame,
    ) -> Result<Value<Box<dyn ValueTypeMarker>>, OpsError> {
        match self {
            UnaryOpcode::Not(_lc) => interpreter::ops::op_unary_not(
                &expr.eval_in_env(frame),
                expr.get_rng(),
                frame.get_file_id(),
            ),
            UnaryOpcode::Neg(_lc) => interpreter::ops::op_unary_neg(
                &expr.eval_in_env(frame),
                expr.get_rng(),
                frame.get_file_id(),
            ),
        }
    }
}

impl AstLoc for UnaryOpcode {
    fn get_begin(&self) -> usize {
        match self {
            UnaryOpcode::Not(loc) | UnaryOpcode::Neg(loc) => loc.get_begin(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            UnaryOpcode::Not(loc) | UnaryOpcode::Neg(loc) => loc.get_end(),
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            UnaryOpcode::Not(loc) | UnaryOpcode::Neg(loc) => loc.as_rng(),
        }
    }
}

pub struct AstFuncCall {
    func_name: (String, TokLoc),
    func_args: AstFuncCallArgs,
    end_pos: usize,
}

impl AstFuncCall {
    pub fn new(name: (String, TokLoc), call_args: AstFuncCallArgs, end_pos: usize) -> AstFuncCall {
        AstFuncCall {
            func_name: name,
            func_args: call_args,
            end_pos,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.func_name.0
    }

    pub fn get_name_loc(&self) -> &TokLoc {
        &self.func_name.1
    }

    pub fn get_args(&self) -> &AstFuncCallArgs {
        &self.func_args
    }
}

impl AstLoc for AstFuncCall {
    #[inline]
    fn get_begin(&self) -> usize {
        self.func_name.1.get_begin()
    }

    #[inline]
    fn get_end(&self) -> usize {
        self.end_pos
    }

    #[inline]
    fn get_rng(&self) -> errors::Location {
        self.get_begin()..self.get_end()
    }
}

pub struct AstFuncCallArgs {
    positional_args: Vec<AstPositionalArg>,
    named_args: Vec<AstNamedExpr>,
}

impl AstFuncCallArgs {
    pub fn new(
        positional_args: Vec<AstPositionalArg>,
        named_args: Vec<AstNamedExpr>,
    ) -> AstFuncCallArgs {
        AstFuncCallArgs {
            positional_args,
            named_args,
        }
    }

    pub fn new_only_positional(positional_args: Vec<AstPositionalArg>) -> AstFuncCallArgs {
        Self::new(positional_args, vec![])
    }

    pub fn new_only_named(named_args: Vec<AstNamedExpr>) -> AstFuncCallArgs {
        Self::new(vec![], named_args)
    }

    pub fn empty() -> AstFuncCallArgs {
        Self::new(vec![], vec![])
    }

    pub fn get_positional_args(&self) -> &Vec<AstPositionalArg> {
        &self.positional_args
    }
    pub fn get_named_args(&self) -> &Vec<AstNamedExpr> {
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

impl AstLoc for AstPositionalArg {
    #[inline]
    fn get_begin(&self) -> usize {
        self.value.get_begin()
    }

    #[inline]
    fn get_end(&self) -> usize {
        self.value.get_end()
    }

    #[inline]
    fn get_rng(&self) -> errors::Location {
        self.value.get_rng()
    }
}

impl From<Box<Expr>> for AstPositionalArg {
    fn from(b: Box<Expr>) -> Self {
        Self { value: *b }
    }
}

pub struct AstNamedExpr {
    name: (String, TokLoc),
    value: Box<Expr>,
}

impl AstNamedExpr {
    pub fn get_name(&self) -> &String {
        &self.name.0
    }
    pub fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl AstLoc for AstNamedExpr {
    #[inline]
    fn get_begin(&self) -> usize {
        self.name.1.get_begin()
    }

    #[inline]
    fn get_end(&self) -> usize {
        self.value.get_end()
    }

    #[inline]
    fn get_rng(&self) -> errors::Location {
        self.get_begin()..self.get_end()
    }
}

impl From<((String, TokLoc), Box<Expr>)> for AstNamedExpr {
    fn from(v: ((String, TokLoc), Box<Expr>)) -> Self {
        let (name, value) = v;
        Self { name, value }
    }
}

pub struct AstMethodCall {
    method_property: AstPropertyAccess,
    args: AstFuncCallArgs,
    end_pos: usize,
}

impl AstMethodCall {
    pub fn new(
        method_property: AstPropertyAccess,
        args: AstFuncCallArgs,
        end_pos: usize,
    ) -> AstMethodCall {
        AstMethodCall {
            method_property,
            args,
            end_pos,
        }
    }

    pub(crate) fn get_base_expr(&self) -> &Expr {
        &self.method_property.base
    }

    #[inline]
    pub(crate) fn get_name(&self) -> &String {
        &self.method_property.get_property_name()
    }

    #[inline]
    pub(crate) fn get_name_loc(&self) -> &TokLoc {
        &self.method_property.get_property_name_loc()
    }

    #[inline]
    pub(crate) fn get_args(&self) -> &AstFuncCallArgs {
        &self.args
    }
}

impl AstLoc for AstMethodCall {
    #[inline]
    fn get_begin(&self) -> usize {
        self.method_property.get_begin()
    }

    #[inline]
    fn get_end(&self) -> usize {
        self.end_pos
    }

    #[inline]
    fn get_rng(&self) -> errors::Location {
        self.get_begin()..self.get_end()
    }
}

pub struct AstAssignment {
    bound_name: Box<Expr>,
    op: AstAtrOp,
    value: Box<Expr>,
}

impl AstAssignment {
    pub fn new(bound_name: Box<Expr>, op: AstAtrOp, value: Box<Expr>) -> AstAssignment {
        AstAssignment {
            bound_name,
            op,
            value,
        }
    }

    #[inline]
    pub fn get_bound(&self) -> &Expr {
        &self.bound_name
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

impl AstLoc for AstAssignment {
    #[inline]
    fn get_begin(&self) -> usize {
        self.bound_name.get_begin()
    }

    #[inline]
    fn get_end(&self) -> usize {
        self.value.get_end()
    }

    #[inline]
    fn get_rng(&self) -> errors::Location {
        self.get_begin()..self.get_end()
    }
}

pub struct AstDeclaration {
    name: (String, TokLoc),
    value: Box<Expr>,
    let_tok: TokLoc,
}

impl AstDeclaration {
    pub fn new(name: (String, TokLoc), value: Box<Expr>, let_tok: TokLoc) -> Self {
        Self {
            name,
            value,
            let_tok,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name.0
    }

    pub fn get_name_loc(&self) -> &TokLoc {
        &self.name.1
    }

    pub fn get_value(&self) -> &Expr {
        &self.value
    }

    pub fn get_let_tok_location(&self) -> &TokLoc {
        &self.let_tok
    }
}

impl AstLoc for AstDeclaration {
    fn get_begin(&self) -> usize {
        self.let_tok.get_begin()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
    }

    fn get_rng(&self) -> errors::Location {
        self.get_begin()..self.get_end()
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
    Declaration(AstDeclaration),
    Assignment(AstAssignment),
}

impl AstLoc for AstStatement {
    #[inline]
    fn get_begin(&self) -> usize {
        match self {
            AstStatement::FuncCall(call) => call.get_begin(),
            AstStatement::MethodCall(call) => call.get_begin(),
            AstStatement::Declaration(decl) => decl.get_begin(),
            AstStatement::Assignment(assignment) => assignment.get_begin(),
        }
    }

    #[inline]
    fn get_end(&self) -> usize {
        match self {
            AstStatement::FuncCall(call) => call.get_end(),
            AstStatement::MethodCall(call) => call.get_end(),
            AstStatement::Declaration(decl) => decl.get_end(),
            AstStatement::Assignment(assignment) => assignment.get_end(),
        }
    }

    #[inline]
    fn get_rng(&self) -> errors::Location {
        self.get_begin()..self.get_end()
    }
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
