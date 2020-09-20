use crate::interpreter::diagnostics::errors::{ExpectedTypeError, VariableNotFoundError};
use crate::interpreter::diagnostics::push_diagnostic;
use crate::interpreter::diagnostics::warnings::KeyAlreadyInMap;
use crate::{
    grammar::lexer::TokLoc,
    interpreter::{
        self,
        diagnostics::{
            self,
            errors::{
                ExprLocAndType, IndexOutsideVectorError, InvalidIndexBaseError, InvalidIndexError,
                TakeRefError,
            },
            Location,
        },
        types::{ErrorValue, TypeId, TypeIdAndValue},
        EnvFrame, LaterValue, ValRefMut, Value, ValueTypeMarker,
    },
};
use std::collections::HashMap;
use std::ops::Deref;

pub(crate) trait AstLoc {
    fn get_begin(&self) -> usize;

    fn get_end(&self) -> usize;

    fn get_rng(&self) -> Location {
        self.get_begin()..self.get_end()
    }
}

/// Present on array and map literal expression
pub(crate) trait IndexedAstLoc {
    fn get_begin_indexed(&self, index: usize) -> usize;
    fn get_end_indexed(&self, index: usize) -> usize;

    fn get_rng_indexed(&self, index: usize) -> Location {
        self.get_begin_indexed(index)..self.get_end_indexed(index)
    }
}

#[derive(Copy, Clone)]
pub enum NumVal {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
}

macro_rules! add_digit_decl {
    ($name:ident, $coef:literal) => {
        fn $name(self, digit: u32) -> Self {
            match self {
                Self::I32(v) => Self::I32(v * $coef + (digit as i32)),
                Self::I64(v) => Self::I64(v * $coef + (digit as i64)),
                Self::U32(v) => Self::U32(v * $coef + (digit as u32)),
                Self::U64(v) => Self::U64(v * $coef + (digit as u64)),
            }
        }
    };
}

impl NumVal {
    fn to_boxed_value(self) -> Box<dyn ValueTypeMarker> {
        match self {
            Self::I32(v) => Box::new(v),
            Self::I64(v) => Box::new(v),
            Self::U32(v) => Box::new(v),
            Self::U64(v) => Box::new(v),
        }
    }

    add_digit_decl! {add_hex_digit, 16}
    add_digit_decl! {add_oct_digit, 8}
    add_digit_decl! {add_bin_digit, 2}
    add_digit_decl! {add_dec_digit, 10}
}

impl From<&str> for NumVal {
    /// parse a number from a number literal string
    fn from(s: &str) -> Self {
        #[derive(Copy, Clone)]
        enum Tp {
            I32,
            I64,
            U32,
            U64,
        }
        impl Tp {
            fn into_unsigned(self) -> Self {
                match self {
                    Self::I32 => Self::U32,
                    Self::I64 => Self::U64,
                    x => x,
                }
            }
            fn into_long(self) -> Self {
                match self {
                    Self::I32 => Self::I64,
                    Self::U32 => Self::U64,
                    x => x,
                }
            }
            fn into_default_num_val(self) -> NumVal {
                match self {
                    Self::I32 => NumVal::I32(0),
                    Self::I64 => NumVal::I64(0),
                    Self::U32 => NumVal::U32(0),
                    Self::U64 => NumVal::U64(0),
                }
            }
        }
        let mut tp = Tp::I32;
        s.chars()
            .rev()
            .take_while(|chr| match chr {
                'u' | 'U' | 'l' | 'L' => true,
                _ => false,
            })
            .for_each(|chr| match chr {
                'u' | 'U' => {
                    tp = tp.into_unsigned();
                }
                'l' | 'L' => {
                    tp = tp.into_long();
                }
                _ => {}
            });
        if s.starts_with("0x") {
            s.chars()
                .skip(2)
                .take_while(|chr| chr.is_digit(16))
                .fold(tp.into_default_num_val(), |nmv, chr| {
                    nmv.add_hex_digit(chr.to_digit(16).unwrap())
                })
        } else if s.starts_with("0b") {
            s.chars()
                .skip(2)
                .take_while(|chr| chr.is_digit(2))
                .fold(tp.into_default_num_val(), |nmv, chr| {
                    nmv.add_bin_digit(chr.to_digit(2).unwrap())
                })
        } else if s.starts_with('0') {
            s.chars()
                .skip(2)
                .take_while(|chr| chr.is_digit(8))
                .fold(tp.into_default_num_val(), |nmv, chr| {
                    nmv.add_oct_digit(chr.to_digit(8).unwrap())
                })
        } else {
            s.chars()
                .skip(2)
                .take_while(|chr| chr.is_digit(10))
                .fold(tp.into_default_num_val(), |nmv, chr| {
                    nmv.add_dec_digit(chr.to_digit(10).unwrap())
                })
        }
    }
}

pub enum Atom {
    Number((NumVal, TokLoc)),
    Bool((bool, TokLoc)),
    Str((String, TokLoc)),
    Id((String, TokLoc)),
    #[allow(clippy::vec_box)] // needed by the way the grammar works
    ArrayLit((Vec<Box<Expr>>, TokLoc)),
    MapLit((Vec<AstNamedExpr>, TokLoc)),
}

impl AstLoc for Atom {
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

    fn get_rng(&self) -> diagnostics::Location {
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
    Indexed {
        base: Box<Expr>,
        index: Box<Expr>,
        end: usize,
    },
    Ternary(
        usize,
        Box<Expr>, //   condition
        Box<Expr>, //           onTrue
        Box<Expr>, //                    onFalse
        usize,
    ),
}

impl Expr {
    pub(crate) fn eval_in_env(&self, frame: &mut EnvFrame) -> Value<Box<dyn ValueTypeMarker>> {
        match self {
            Expr::Atom(Atom::Number((num, _loc))) => Value::new(num.to_boxed_value()),
            Expr::Atom(Atom::Bool((v, _loc))) => Value::new(Box::new(*v)),
            Expr::Atom(Atom::Str((str, _loc))) => Value::new(Box::new(str.clone())),
            Expr::Atom(Atom::Id((id, loc))) => match frame.get_value_for_variable(&id.clone()) {
                Some(v) => v.clone_to_value(),
                None => {
                    diagnostics::push_diagnostic(VariableNotFoundError::new(loc.as_rng()), frame);
                    Value::new(Box::new(ErrorValue::new()))
                }
            },
            Expr::Atom(Atom::ArrayLit((v, _loc))) => {
                let mut val = Vec::with_capacity(v.capacity());
                v.iter().for_each(|x| val.push(x.eval_in_env(frame)));
                Value::new(Box::new(val))
            }
            Expr::Atom(Atom::MapLit((v, map_lit_loc))) => {
                let mut val = HashMap::with_capacity(v.capacity());
                v.iter().for_each(|x: &AstNamedExpr| {
                    if val
                        .insert(x.name.0.clone(), x.value.eval_in_env(frame))
                        .is_some()
                    {
                        diagnostics::push_diagnostic(
                            KeyAlreadyInMap::new(&x.name.0, map_lit_loc.as_rng(), x.get_rng()),
                            frame,
                        );
                    }
                });
                Value::new(Box::new(val))
            }
            Expr::FuncCall(call_expr) => interpreter::func_call_result(call_expr, frame),
            Expr::MethodCall(call_expr) => {
                interpreter::method_call_result(&call_expr.method_property, &call_expr.args, frame)
            }
            Expr::Op(left, opcode, right) => opcode.compute_result_for(left, right, frame),
            Expr::UnaryOp(opcode, right) => opcode.compute_result_for(right, frame),
            Expr::PropertyAccess(access) => interpreter::property_access(access, frame),
            Expr::ParenExpr(_, e, _) => e.eval_in_env(frame),
            Expr::Indexed {
                base,
                index: index_expr,
                ..
            } => Value::new(Box::new(
                match base.eval_in_env(frame).get_type_id_and_value() {
                    TypeIdAndValue::Vec(v) => {
                        let index_result = index_expr
                            .eval_in_env(frame)
                            .get_value()
                            .get_type_id_and_value()
                            .cast_to_usize();
                        match index_result {
                            Ok(index) => {
                                if index < v.len() {
                                    v[index].clone_to_value()
                                } else {
                                    diagnostics::push_diagnostic(
                                        IndexOutsideVectorError::new(
                                            base.get_rng(),
                                            index_expr.get_rng(),
                                            v.len(),
                                            index,
                                        ),
                                        frame,
                                    );
                                    return Value::new(Box::new(ErrorValue::new()));
                                }
                            }
                            Err(t) => {
                                diagnostics::push_diagnostic(
                                    InvalidIndexError::new(
                                        ExprLocAndType::new(base.get_rng(), TypeId::Vec.typename()),
                                        ExprLocAndType::new(index_expr.get_rng(), t.typename()),
                                    ),
                                    frame,
                                );
                                return Value::new(Box::new(ErrorValue::new()));
                            }
                        }
                    }
                    TypeIdAndValue::Map(v) => {
                        let index_val = index_expr.eval_in_env(frame);
                        let index_typeid_and_value = index_val.get_value().get_type_id_and_value();
                        let index_result = index_typeid_and_value.get_string();
                        match index_result {
                            Ok(index) => v[index].clone_to_value(),
                            Err(t) => {
                                diagnostics::push_diagnostic(
                                    InvalidIndexError::new(
                                        ExprLocAndType::new(base.get_rng(), TypeId::Vec.typename()),
                                        ExprLocAndType::new(index_expr.get_rng(), t.typename()),
                                    ),
                                    frame,
                                );
                                return Value::new(Box::new(ErrorValue::new()));
                            }
                        }
                    }
                    t => {
                        diagnostics::push_diagnostic(
                            InvalidIndexBaseError::new(ExprLocAndType::new(
                                base.get_rng(),
                                t.degrade().typename(),
                            )),
                            frame,
                        );
                        return Value::new(Box::new(ErrorValue::new()));
                    }
                },
            )),
            Expr::Ternary(_, condition, value_true, value_false, _) => {
                let cond = condition.eval_in_env(frame);
                match cond.get_type_id_and_value_required(TypeId::Bool) {
                    Ok(b) => {
                        /*safe to unwrap here*/
                        if b.get_bool().unwrap() {
                            value_true.eval_in_env(frame)
                        } else {
                            value_false.eval_in_env(frame)
                        }
                    }
                    Err(tp) => {
                        push_diagnostic(
                            ExpectedTypeError::new(
                                TypeId::Bool.typename(),
                                ExprLocAndType::new(condition.get_rng(), tp.typename()),
                            ),
                            frame,
                        );
                        Value::new(Box::new(ErrorValue::new()))
                    }
                }
            }
        }
    }

    pub(crate) fn eval_mut_ref<'a>(
        &self,
        frame: &'a mut EnvFrame,
    ) -> Result<ValRefMut<'a, Box<dyn ValueTypeMarker>>, TakeRefError> {
        match self {
            Expr::Atom(atom) => match atom {
                Atom::Id((name, _)) => match frame.get_variables_mut().get_mut(name) {
                    Some(v) => Ok(ValRefMut::new(v.get_value_mut())),
                    None => Err(TakeRefError::new(self.get_rng(), "a non-existent variable")),
                },
                Atom::Number((_, loc)) => Err(TakeRefError::new(loc.as_rng(), "a number literal")),
                Atom::Str((_, loc)) => Err(TakeRefError::new(loc.as_rng(), "a string literal")),
                Atom::Bool((_v, loc)) => Err(TakeRefError::new(loc.as_rng(), "a bool literal")),
                Atom::ArrayLit((_v, loc)) => {
                    Err(TakeRefError::new(loc.as_rng(), "an array literal"))
                }
                Atom::MapLit((_v, loc)) => Err(TakeRefError::new(loc.as_rng(), "a map literal")),
            },
            Expr::FuncCall(_) => Err(TakeRefError::new(self.get_rng(), "a function call")),
            Expr::MethodCall(_) => Err(TakeRefError::new(self.get_rng(), "a method call")),
            Expr::PropertyAccess(_) => Err(TakeRefError::new(self.get_rng(), "a property")),
            Expr::Op(_, _, _) => Err(TakeRefError::new(
                self.get_rng(),
                "an arithmetic expression",
            )),
            Expr::UnaryOp(_, _) => {
                Err(TakeRefError::new(self.get_rng(), "an expression like this"))
            }
            Expr::ParenExpr(_, e, _) => e.eval_mut_ref(frame),
            Expr::Indexed { .. } => {
                Err(TakeRefError::new(self.get_rng(), "an expression like this"))
            }
            Expr::Ternary(_, _, _, _, _) => {
                Err(TakeRefError::new(self.get_rng(), "an expression like this"))
            }
        }
    }

    fn later_eval(&self) -> LaterValue {
        LaterValue::new(self)
    }
}

impl AstLoc for Expr {
    fn get_begin(&self) -> usize {
        match self {
            Expr::Atom(atm) => atm.get_begin(),
            Expr::FuncCall(call) => call.get_begin(),
            Expr::MethodCall(call) => call.get_begin(),
            Expr::Op(expr, _, _) => expr.get_begin(),
            Expr::PropertyAccess(prop_access) => prop_access.get_begin(),
            Expr::ParenExpr(begin, _, _) => *begin,
            Expr::UnaryOp(op, _) => op.get_begin(),
            Expr::Indexed { base, .. } => base.get_begin(),
            Expr::Ternary(b, _, _, _, _) => *b,
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Expr::Atom(atm) => atm.get_end(),
            Expr::FuncCall(call) => call.get_end(),
            Expr::MethodCall(call) => call.get_end(),
            Expr::Op(expr, _, _) => expr.get_end(),
            Expr::PropertyAccess(prop_access) => prop_access.get_end(),
            Expr::ParenExpr(_, _, end) => *end,
            Expr::UnaryOp(_, expr) => expr.get_end(),
            Expr::Indexed { end, .. } => *end,
            Expr::Ternary(_, _, _, _, e) => *e,
        }
    }

    fn get_rng(&self) -> diagnostics::Location {
        match self {
            Expr::Atom(atm) => atm.get_rng(),
            Expr::FuncCall(call) => call.get_rng(),
            Expr::MethodCall(call) => call.get_rng(),
            Expr::Op(expr1, _, expr2) => expr1.get_begin()..expr2.get_end(),
            Expr::UnaryOp(op, expr) => op.get_begin()..expr.get_end(),
            Expr::PropertyAccess(prop_access) => prop_access.get_rng(),
            Expr::ParenExpr(begin, _, end) => *begin..*end,
            Expr::Indexed { base: b, end, .. } => b.get_begin()..*end,
            Expr::Ternary(b, _, _, _, e) => (*b)..(*e),
        }
    }
}

impl IndexedAstLoc for Expr {
    fn get_begin_indexed(&self, index: usize) -> usize {
        match self {
            Expr::Atom(Atom::ArrayLit((lit, _))) => lit[index].get_begin(),
            Expr::Atom(Atom::MapLit((lit, _))) => lit[index].get_begin(),
            _ => self.get_begin(),
        }
    }

    fn get_end_indexed(&self, index: usize) -> usize {
        match self {
            Expr::Atom(Atom::ArrayLit((lit, _))) => lit[index].get_end(),
            Expr::Atom(Atom::MapLit((lit, _))) => lit[index].get_end(),
            _ => self.get_end(),
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

    pub(crate) fn get_base(&self) -> &Expr {
        &self.base
    }

    pub(crate) fn get_property_name(&self) -> &String {
        &self.property_name.0
    }

    pub(crate) fn get_property_name_loc(&self) -> &TokLoc {
        &self.property_name.1
    }
}

impl AstLoc for AstPropertyAccess {
    fn get_begin(&self) -> usize {
        self.base.get_begin()
    }

    fn get_end(&self) -> usize {
        self.property_name.1.get_end()
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
    ) -> Value<Box<dyn ValueTypeMarker>> {
        match self {
            Opcode::Add => match interpreter::ops::op_add(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
            ) {
                (v, Ok(())) => v,
                (v, Err(diagnostic)) => {
                    diagnostics::push_diagnostic(diagnostic, frame);
                    v
                }
            },
            Opcode::Sub => match interpreter::ops::op_sub(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
            ) {
                (v, Ok(())) => v,
                (v, Err(diagnostic)) => {
                    diagnostics::push_diagnostic(diagnostic, frame);
                    v
                }
            },
            Opcode::Mul => match interpreter::ops::op_mul(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
            ) {
                (v, Ok(())) => v,
                (v, Err(diagnostic)) => {
                    diagnostics::push_diagnostic(diagnostic, frame);
                    v
                }
            },
            Opcode::Div => match interpreter::ops::op_div(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
            ) {
                (v, Ok(())) => v,
                (v, Err(diagnostic)) => {
                    diagnostics::push_diagnostic(diagnostic, frame);
                    v
                }
            },
            Opcode::Mod => match interpreter::ops::op_mod(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
            ) {
                (v, Ok(())) => v,
                (v, Err(diagnostic)) => {
                    diagnostics::push_diagnostic(diagnostic, frame);
                    v
                }
            },
            Opcode::And => interpreter::ops::op_and(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.later_eval(),
                rs.get_rng(),
                frame,
            ),
            Opcode::Or => interpreter::ops::op_or(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.later_eval(),
                rs.get_rng(),
                frame,
            ),
            Opcode::In => Value::new(Box::new(())),
            Opcode::NotIn => Value::new(Box::new(())),
            Opcode::Equal => interpreter::ops::op_eq(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_diagnostics_ctx(),
            ),
            Opcode::G => interpreter::ops::op_g(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_diagnostics_ctx(),
            ),
            Opcode::L => interpreter::ops::op_l(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_diagnostics_ctx(),
            ),
            Opcode::GE => interpreter::ops::op_ge(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_diagnostics_ctx(),
            ),
            Opcode::LE => interpreter::ops::op_le(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_diagnostics_ctx(),
            ),
            Opcode::NE => interpreter::ops::op_neq(
                &ls.eval_in_env(frame),
                ls.get_rng(),
                &rs.eval_in_env(frame),
                rs.get_rng(),
                frame.get_diagnostics_ctx(),
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
    ) -> Value<Box<dyn ValueTypeMarker>> {
        match self {
            UnaryOpcode::Not(_lc) => interpreter::ops::op_unary_not(
                &expr.eval_in_env(frame),
                expr.get_rng(),
                frame.get_diagnostics_ctx(),
            ),
            UnaryOpcode::Neg(_lc) => interpreter::ops::op_unary_neg(
                &expr.eval_in_env(frame),
                expr.get_rng(),
                frame.get_diagnostics_ctx(),
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
    fn get_begin(&self) -> usize {
        self.func_name.1.get_begin()
    }

    fn get_end(&self) -> usize {
        self.end_pos
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
    fn get_begin(&self) -> usize {
        self.value.get_begin()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
    }

    fn get_rng(&self) -> diagnostics::Location {
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
    fn get_begin(&self) -> usize {
        self.name.1.get_begin()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
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

    pub(crate) fn get_name(&self) -> &String {
        &self.method_property.get_property_name()
    }

    pub(crate) fn get_name_loc(&self) -> &TokLoc {
        &self.method_property.get_property_name_loc()
    }

    pub(crate) fn get_args(&self) -> &AstFuncCallArgs {
        &self.args
    }
}

impl AstLoc for AstMethodCall {
    fn get_begin(&self) -> usize {
        self.method_property.get_begin()
    }

    fn get_end(&self) -> usize {
        self.end_pos
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

    pub fn get_bound(&self) -> &Expr {
        &self.bound_name
    }

    pub fn get_op(&self) -> &AstAtrOp {
        &self.op
    }

    pub fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl AstLoc for AstAssignment {
    fn get_begin(&self) -> usize {
        self.bound_name.get_begin()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
    }
}

pub struct AstDeclaration {
    name: (String, TokLoc),
    value: Box<Expr>,
    begin: usize,
}

impl AstDeclaration {
    pub fn new(name: (String, TokLoc), value: Box<Expr>, begin: usize) -> Self {
        Self { name, value, begin }
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
}

impl AstLoc for AstDeclaration {
    fn get_begin(&self) -> usize {
        self.begin
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
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

pub struct AstIf {
    /// index of "if"
    begin: usize,
    condition: Box<Expr>,
    statements: Vec<AstStatement>,

    end: usize,
}

impl AstIf {
    pub(crate) fn new(
        begin: usize,
        condition: Box<Expr>,
        statements: Vec<AstStatement>,
        end: usize,
    ) -> Self {
        Self {
            begin,
            condition,
            statements,
            end,
        }
    }

    pub(crate) fn get_condition(&self) -> &Expr {
        &self.condition
    }
    pub(crate) fn get_statements(&self) -> &Vec<AstStatement> {
        &self.statements
    }
}

impl AstLoc for AstIf {
    fn get_begin(&self) -> usize {
        self.begin
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

pub struct AstElseIf {
    /// start index of "else"
    begin: usize,
    if_: AstIf,
}

impl AstLoc for AstElseIf {
    fn get_begin(&self) -> usize {
        self.begin
    }

    fn get_end(&self) -> usize {
        self.if_.get_end()
    }
}

impl AstElseIf {
    pub(crate) fn new(begin: usize, if_: AstIf) -> Self {
        Self { begin, if_ }
    }

    pub(crate) fn get_if(&self) -> &AstIf {
        &self.if_
    }
}

pub struct AstElse {
    /// index of "else"
    begin: usize,
    statements: Vec<AstStatement>,
    end: usize,
}

impl AstLoc for AstElse {
    fn get_begin(&self) -> usize {
        self.begin
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

impl AstElse {
    pub(crate) fn new(begin: usize, statements: Vec<AstStatement>, end: usize) -> Self {
        Self {
            begin,
            statements,
            end,
        }
    }

    pub(crate) fn get_statements(&self) -> &Vec<AstStatement> {
        &self.statements
    }
}

pub struct AstConditionalStatement {
    initial_if: Box<AstIf>,
    else_ifs: Vec<AstElseIf>,
    else_: Box<Option<AstElse>>,
}

impl AstConditionalStatement {
    pub(crate) fn new(initial_if: AstIf, else_ifs: Vec<AstElseIf>, else_: Option<AstElse>) -> Self {
        Self {
            initial_if: Box::new(initial_if),
            else_ifs,
            else_: Box::new(else_),
        }
    }

    pub(crate) fn get_initial_if(&self) -> &AstIf {
        &self.initial_if
    }
    pub(crate) fn get_else_ifs(&self) -> &Vec<AstElseIf> {
        &self.else_ifs
    }
    pub(crate) fn get_else(&self) -> &Option<AstElse> {
        &self.else_
    }
}

impl AstLoc for AstConditionalStatement {
    fn get_begin(&self) -> usize {
        self.initial_if.get_begin()
    }

    fn get_end(&self) -> usize {
        match self.else_.deref() {
            Some(els) => els.get_end(),
            None => match self.else_ifs.last() {
                Some(else_if) => else_if.get_end(),
                None => self.initial_if.get_end(),
            },
        }
    }
}

pub struct AstRepetitiveStatement {
    foreach_tok: usize,
    for_in_expr: AstForInExpr,
    statements: Vec<AstStatement>,
    end: usize,
}

impl AstRepetitiveStatement {
    pub fn new(
        foreach_tok: usize,
        for_in_expr: AstForInExpr,
        statements: Vec<AstStatement>,
        end: usize,
    ) -> AstRepetitiveStatement {
        Self {
            foreach_tok,
            for_in_expr,
            statements,
            end,
        }
    }

    pub(crate) fn get_for_in_expr(&self) -> &AstForInExpr {
        &self.for_in_expr
    }
    pub(crate) fn get_statements(&self) -> &Vec<AstStatement> {
        &self.statements
    }
}

impl AstLoc for AstRepetitiveStatement {
    fn get_begin(&self) -> usize {
        self.for_in_expr.get_begin()
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

pub struct AstForInExpr {
    name: (String, TokLoc),
    expr: Box<Expr>,
}

impl AstForInExpr {
    pub fn new(name: (String, TokLoc), expr: Box<Expr>) -> AstForInExpr {
        Self { name, expr }
    }

    pub(crate) fn get_name(&self) -> &(String, TokLoc) {
        &self.name
    }

    pub(crate) fn get_expr(&self) -> &Expr {
        &self.expr
    }
}

impl AstLoc for AstForInExpr {
    fn get_begin(&self) -> usize {
        self.name.1.get_begin()
    }

    fn get_end(&self) -> usize {
        self.expr.get_end()
    }
}

pub enum AstControlStatement {
    Continue(TokLoc),
    Break(TokLoc),
}

impl AstLoc for AstControlStatement {
    fn get_begin(&self) -> usize {
        match self {
            AstControlStatement::Continue(loc) | AstControlStatement::Break(loc) => loc.get_begin(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            AstControlStatement::Continue(loc) | AstControlStatement::Break(loc) => loc.get_end(),
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            AstControlStatement::Continue(loc) | AstControlStatement::Break(loc) => loc.as_rng(),
        }
    }
}

pub enum AstStatement {
    FuncCall(AstFuncCall),
    MethodCall(AstMethodCall),
    Declaration(AstDeclaration),
    Assignment(AstAssignment),
    Conditional(AstConditionalStatement),
    ControlStatement(AstControlStatement),
    Repetitive(AstRepetitiveStatement),
}

impl AstLoc for AstStatement {
    fn get_begin(&self) -> usize {
        match self {
            AstStatement::FuncCall(call) => call.get_begin(),
            AstStatement::MethodCall(call) => call.get_begin(),
            AstStatement::Declaration(decl) => decl.get_begin(),
            AstStatement::Assignment(assignment) => assignment.get_begin(),
            AstStatement::ControlStatement(control_statement) => control_statement.get_begin(),
            AstStatement::Conditional(conditional_statement) => conditional_statement.get_begin(),
            AstStatement::Repetitive(repetitive) => repetitive.get_begin(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            AstStatement::FuncCall(call) => call.get_end(),
            AstStatement::MethodCall(call) => call.get_end(),
            AstStatement::Declaration(decl) => decl.get_end(),
            AstStatement::Assignment(assignment) => assignment.get_end(),
            AstStatement::ControlStatement(control_statement) => control_statement.get_end(),
            AstStatement::Conditional(conditional_statement) => conditional_statement.get_end(),
            AstStatement::Repetitive(repetitive) => repetitive.get_end(),
        }
    }
}

impl<T> AstLoc for Vec<T>
where
    T: AstLoc,
{
    fn get_begin(&self) -> usize {
        match self.first() {
            Some(l) => l.get_begin(),
            None => 0,
        }
    }

    fn get_end(&self) -> usize {
        match self.last() {
            Some(l) => l.get_end(),
            None => 0,
        }
    }
}

impl<T> AstLoc for &[T]
where
    T: AstLoc,
{
    fn get_begin(&self) -> usize {
        match self.first() {
            Some(l) => l.get_begin(),
            None => 0,
        }
    }

    fn get_end(&self) -> usize {
        match self.last() {
            Some(l) => l.get_end(),
            None => 0,
        }
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
