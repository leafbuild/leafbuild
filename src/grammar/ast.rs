use crate::grammar::lexer::Span;
use std::any::Any;
use std::ops::{Deref, Range};

type Location = Range<usize>;

pub(crate) trait AstLoc {
    fn get_start(&self) -> usize;

    fn get_end(&self) -> usize;

    fn get_rng(&self) -> Location {
        self.get_start()..self.get_end()
    }
}

#[derive(Copy, Clone, Debug)]
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
    fn to_boxed_value(self) -> Box<dyn Any> {
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
            .take_while(|chr| matches!(chr, 'u' | 'U' | 'l' | 'L'))
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
                .skip(1)
                .take_while(|chr| chr.is_digit(8))
                .fold(tp.into_default_num_val(), |nmv, chr| {
                    nmv.add_oct_digit(chr.to_digit(8).unwrap())
                })
        } else {
            s.chars()
                .take_while(|chr| chr.is_digit(10))
                .fold(tp.into_default_num_val(), |nmv, chr| {
                    nmv.add_dec_digit(chr.to_digit(10).unwrap())
                })
        }
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Number((NumVal, Span)),
    Bool((bool, Span)),
    Str((String, Span)),
    Id((String, Span)),
    ArrayLit((Vec<Box<Expr>>, Span, Span)),
    MapLit((Vec<AstNamedExpr>, Span, Span)),
}

impl AstLoc for Atom {
    fn get_start(&self) -> usize {
        match self {
            Atom::Bool((_, loc))
            | Atom::Number((_, loc))
            | Atom::Id((_, loc))
            | Atom::Str((_, loc)) => loc.get_start(),
            Atom::ArrayLit((_, lbrace, _)) | Atom::MapLit((_, lbrace, _)) => lbrace.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Atom::Bool((_, loc))
            | Atom::Number((_, loc))
            | Atom::Id((_, loc))
            | Atom::Str((_, loc)) => loc.get_end(),
            Atom::ArrayLit((_, _, rbrace)) | Atom::MapLit((_, _, rbrace)) => rbrace.get_end(),
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            Atom::Bool((_, loc))
            | Atom::Number((_, loc))
            | Atom::Id((_, loc))
            | Atom::Str((_, loc)) => loc.as_rng(),
            Atom::ArrayLit((_, lbrace, rbrace)) | Atom::MapLit((_, lbrace, rbrace)) => {
                lbrace.get_start()..rbrace.get_end()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Atom),
    Op(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(UnaryOpcode, Box<Expr>),
    FuncCall(AstFuncCall),
    MethodCall(AstMethodCall),
    PropertyAccess(AstPropertyAccess),
    ParenExpr {
        lparen: Span,
        expr: Box<Expr>,
        rparen: Span,
    },
    Indexed {
        base: Box<Expr>,
        open_bracket: Span,
        index: Box<Expr>,
        close_bracket: Span,
    },
    Ternary {
        condition: Box<Expr>,
        qmark: Span,
        if_true: Box<Expr>,
        colon: Span,
        if_false: Box<Expr>,
    },
}

pub(crate) struct Spanned<T> {
    data: T,
    location: Span,
}

impl Expr {}

impl AstLoc for Expr {
    fn get_start(&self) -> usize {
        match self {
            Expr::Atom(atm) => atm.get_start(),
            Expr::FuncCall(call) => call.get_start(),
            Expr::MethodCall(call) => call.get_start(),
            Expr::Op(expr, _, _) => expr.get_start(),
            Expr::PropertyAccess(prop_access) => prop_access.get_start(),
            Expr::ParenExpr { lparen, .. } => lparen.get_start(),
            Expr::UnaryOp(op, _) => op.get_start(),
            Expr::Indexed { base, .. } => base.get_start(),
            Expr::Ternary { condition, .. } => condition.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Expr::Atom(atm) => atm.get_end(),
            Expr::FuncCall(call) => call.get_end(),
            Expr::MethodCall(call) => call.get_end(),
            Expr::Op(expr, _, _) => expr.get_end(),
            Expr::PropertyAccess(prop_access) => prop_access.get_end(),
            Expr::ParenExpr { rparen, .. } => rparen.get_end(),
            Expr::UnaryOp(_, expr) => expr.get_end(),
            Expr::Indexed { close_bracket, .. } => close_bracket.get_end(),
            Expr::Ternary { if_false, .. } => if_false.get_end(),
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            Expr::Atom(atm) => atm.get_rng(),
            Expr::FuncCall(call) => call.get_rng(),
            Expr::MethodCall(call) => call.get_rng(),
            Expr::Op(expr1, _, expr2) => expr1.get_start()..expr2.get_end(),
            Expr::UnaryOp(op, expr) => op.get_start()..expr.get_end(),
            Expr::PropertyAccess(prop_access) => prop_access.get_rng(),
            Expr::ParenExpr { lparen, rparen, .. } => lparen.get_start()..rparen.get_end(),
            Expr::Indexed {
                base,
                close_bracket,
                ..
            } => base.get_start()..close_bracket.get_end(),
            Expr::Ternary {
                condition,
                if_false,
                ..
            } => condition.get_start()..if_false.get_end(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstPropertyAccess {
    base: Box<Expr>,
    property_name: (String, Span),
}

impl AstPropertyAccess {
    pub fn new(base: Box<Expr>, property_name: (String, Span)) -> Self {
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

    pub(crate) fn get_property_name_loc(&self) -> &Span {
        &self.property_name.1
    }
}

impl AstLoc for AstPropertyAccess {
    fn get_start(&self) -> usize {
        self.base.get_start()
    }

    fn get_end(&self) -> usize {
        self.property_name.1.get_end()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Mul(Span),
    Div(Span),
    Add(Span),
    Sub(Span),
    Mod(Span),
    And(Span),
    Or(Span),
    In(Span),
    NotIn(Span),
    Equal(Span),
    G(Span),
    L(Span),
    GE(Span),
    LE(Span),
    NE(Span),
    LBitshift(Span),
    RBitshift(Span),
}

impl Opcode {}

impl AstLoc for Opcode {
    fn get_start(&self) -> usize {
        match self {
            Opcode::Mul(loc)
            | Opcode::Div(loc)
            | Opcode::Add(loc)
            | Opcode::Sub(loc)
            | Opcode::Mod(loc)
            | Opcode::And(loc)
            | Opcode::Or(loc)
            | Opcode::In(loc)
            | Opcode::NotIn(loc)
            | Opcode::Equal(loc)
            | Opcode::G(loc)
            | Opcode::L(loc)
            | Opcode::GE(loc)
            | Opcode::LE(loc)
            | Opcode::NE(loc)
            | Opcode::LBitshift(loc)
            | Opcode::RBitshift(loc) => loc.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Opcode::Mul(loc)
            | Opcode::Div(loc)
            | Opcode::Add(loc)
            | Opcode::Sub(loc)
            | Opcode::Mod(loc)
            | Opcode::And(loc)
            | Opcode::Or(loc)
            | Opcode::In(loc)
            | Opcode::NotIn(loc)
            | Opcode::Equal(loc)
            | Opcode::G(loc)
            | Opcode::L(loc)
            | Opcode::GE(loc)
            | Opcode::LE(loc)
            | Opcode::NE(loc)
            | Opcode::LBitshift(loc)
            | Opcode::RBitshift(loc) => loc.get_end(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOpcode {
    Plus(Span),
    Minus(Span),
    Not(Span),
    BitwiseNot(Span),
}

impl UnaryOpcode {}

impl AstLoc for UnaryOpcode {
    fn get_start(&self) -> usize {
        match self {
            UnaryOpcode::BitwiseNot(loc)
            | UnaryOpcode::Not(loc)
            | UnaryOpcode::Plus(loc)
            | UnaryOpcode::Minus(loc) => loc.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            UnaryOpcode::BitwiseNot(loc)
            | UnaryOpcode::Not(loc)
            | UnaryOpcode::Plus(loc)
            | UnaryOpcode::Minus(loc) => loc.get_end(),
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            UnaryOpcode::BitwiseNot(loc)
            | UnaryOpcode::Not(loc)
            | UnaryOpcode::Plus(loc)
            | UnaryOpcode::Minus(loc) => loc.as_rng(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstFuncCall {
    func_name: (String, Span),
    func_args: AstFuncCallArgs,
    end_pos: usize,
}

impl AstFuncCall {
    pub fn new(name: (String, Span), call_args: AstFuncCallArgs, end_pos: usize) -> AstFuncCall {
        AstFuncCall {
            func_name: name,
            func_args: call_args,
            end_pos,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.func_name.0
    }

    pub fn get_name_loc(&self) -> &Span {
        &self.func_name.1
    }

    pub fn get_args(&self) -> &AstFuncCallArgs {
        &self.func_args
    }
}

impl AstLoc for AstFuncCall {
    fn get_start(&self) -> usize {
        self.func_name.1.get_start()
    }

    fn get_end(&self) -> usize {
        self.end_pos
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct AstPositionalArg {
    value: Expr,
}

impl AstPositionalArg {
    pub(crate) fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl AstLoc for AstPositionalArg {
    fn get_start(&self) -> usize {
        self.value.get_start()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
    }

    fn get_rng(&self) -> Location {
        self.value.get_rng()
    }
}

impl From<Box<Expr>> for AstPositionalArg {
    fn from(b: Box<Expr>) -> Self {
        Self { value: *b }
    }
}

#[derive(Debug, Clone)]
pub struct AstNamedExpr {
    name: (String, Span),
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
    fn get_start(&self) -> usize {
        self.name.1.get_start()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
    }
}

impl From<((String, Span), Box<Expr>)> for AstNamedExpr {
    fn from(v: ((String, Span), Box<Expr>)) -> Self {
        let (name, value) = v;
        Self { name, value }
    }
}

#[derive(Debug, Clone)]
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

    pub(crate) fn get_name_loc(&self) -> &Span {
        &self.method_property.get_property_name_loc()
    }

    pub(crate) fn get_args(&self) -> &AstFuncCallArgs {
        &self.args
    }
}

impl AstLoc for AstMethodCall {
    fn get_start(&self) -> usize {
        self.method_property.get_start()
    }

    fn get_end(&self) -> usize {
        self.end_pos
    }
}

#[derive(Debug, Clone)]
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
    fn get_start(&self) -> usize {
        self.bound_name.get_start()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
    }
}

#[derive(Debug, Clone)]
pub struct AstDeclaration {
    let_tok: Span,
    name: (String, Span),
    eq: Span,
    value: Box<Expr>,
}

impl AstDeclaration {
    pub fn new(let_tok: Span, name: (String, Span), eq: Span, value: Box<Expr>) -> Self {
        Self {
            let_tok,
            name,
            eq,
            value,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name.0
    }

    pub fn get_name_loc(&self) -> &Span {
        &self.name.1
    }

    pub fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl AstLoc for AstDeclaration {
    fn get_start(&self) -> usize {
        self.let_tok.get_start()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
    }
}

#[derive(Debug, Clone)]
pub enum AstAtrOp {
    Atr(Span),
    AddAtr(Span),
    SubAtr(Span),
    MulAtr(Span),
    DivAtr(Span),
    ModAtr(Span),
}

#[derive(Debug, Clone)]
pub struct AstIf {
    if_tok: Span,
    condition: Box<Expr>,
    lbrace: Span,
    statements: Vec<AstStatement>,
    rbrace: Span,
}

impl AstIf {
    pub(crate) fn new(
        if_tok: Span,
        condition: Box<Expr>,
        lbrace: Span,
        statements: Vec<AstStatement>,
        rbrace: Span,
    ) -> Self {
        Self {
            if_tok,
            condition,
            lbrace,
            statements,
            rbrace,
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
    fn get_start(&self) -> usize {
        self.if_tok.get_start()
    }

    fn get_end(&self) -> usize {
        self.rbrace.get_end()
    }
}

#[derive(Debug, Clone)]
pub struct AstElseIf {
    else_tok: Span,
    if_: AstIf,
}

impl AstLoc for AstElseIf {
    fn get_start(&self) -> usize {
        self.else_tok.get_start()
    }

    fn get_end(&self) -> usize {
        self.if_.get_end()
    }
}

impl AstElseIf {
    pub(crate) fn new(else_tok: Span, if_: AstIf) -> Self {
        Self { else_tok, if_ }
    }

    pub(crate) fn get_if(&self) -> &AstIf {
        &self.if_
    }
}

#[derive(Debug, Clone)]
pub struct AstElse {
    else_tok: Span,
    lbrace: Span,
    statements: Vec<AstStatement>,
    rbrace: Span,
}

impl AstLoc for AstElse {
    fn get_start(&self) -> usize {
        self.else_tok.get_start()
    }

    fn get_end(&self) -> usize {
        self.rbrace.get_end()
    }
}

impl AstElse {
    pub(crate) fn new(
        else_tok: Span,
        lbrace: Span,
        statements: Vec<AstStatement>,
        rbrace: Span,
    ) -> Self {
        Self {
            else_tok,
            lbrace,
            statements,
            rbrace,
        }
    }

    pub(crate) fn get_statements(&self) -> &Vec<AstStatement> {
        &self.statements
    }
}

#[derive(Debug, Clone)]
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
    fn get_start(&self) -> usize {
        self.initial_if.get_start()
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

#[derive(Debug, Clone)]
pub struct AstRepetitiveStatement {
    foreach_tok: Span,
    for_in_expr: AstForInExpr,
    lbrace: Span,
    statements: Vec<AstStatement>,
    rbrace: Span,
}

impl AstRepetitiveStatement {
    pub fn new(
        foreach_tok: Span,
        for_in_expr: AstForInExpr,
        lbrace: Span,
        statements: Vec<AstStatement>,
        rbrace: Span,
    ) -> AstRepetitiveStatement {
        Self {
            foreach_tok,
            for_in_expr,
            lbrace,
            statements,
            rbrace,
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
    fn get_start(&self) -> usize {
        self.for_in_expr.get_start()
    }

    fn get_end(&self) -> usize {
        self.rbrace.get_end()
    }
}

#[derive(Debug, Clone)]
pub struct AstForInExpr {
    name: (String, Span),
    in_tok: Span,
    expr: Box<Expr>,
}

impl AstForInExpr {
    pub fn new(name: (String, Span), in_tok: Span, expr: Box<Expr>) -> AstForInExpr {
        Self { name, in_tok, expr }
    }

    pub(crate) fn get_name(&self) -> &(String, Span) {
        &self.name
    }

    pub(crate) fn get_expr(&self) -> &Expr {
        &self.expr
    }
}

impl AstLoc for AstForInExpr {
    fn get_start(&self) -> usize {
        self.name.1.get_start()
    }

    fn get_end(&self) -> usize {
        self.expr.get_end()
    }
}

#[derive(Debug, Clone)]
pub enum AstControlStatement {
    Continue(Span),
    Break(Span),
}

impl AstLoc for AstControlStatement {
    fn get_start(&self) -> usize {
        match self {
            AstControlStatement::Continue(loc) | AstControlStatement::Break(loc) => loc.get_start(),
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

#[derive(Debug, Clone)]
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
    fn get_start(&self) -> usize {
        match self {
            AstStatement::FuncCall(call) => call.get_start(),
            AstStatement::MethodCall(call) => call.get_start(),
            AstStatement::Declaration(decl) => decl.get_start(),
            AstStatement::Assignment(assignment) => assignment.get_start(),
            AstStatement::ControlStatement(control_statement) => control_statement.get_start(),
            AstStatement::Conditional(conditional_statement) => conditional_statement.get_start(),
            AstStatement::Repetitive(repetitive) => repetitive.get_start(),
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
    fn get_start(&self) -> usize {
        match self.first() {
            Some(l) => l.get_start(),
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
    fn get_start(&self) -> usize {
        match self.first() {
            Some(l) => l.get_start(),
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

#[derive(Debug, Clone)]
pub struct AstBuildDefinition {
    statements: Vec<AstStatement>,
}

impl AstBuildDefinition {
    pub fn new(statements: Vec<AstStatement>) -> Self {
        Self { statements }
    }
    pub fn get_statements(&self) -> &Vec<AstStatement> {
        &self.statements
    }
}
