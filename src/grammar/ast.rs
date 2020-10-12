use crate::grammar::lexer::TokLoc;
use std::any::Any;
use std::ops::{Deref, Range};

type Location = Range<usize>;

pub(crate) trait AstLoc {
    fn get_begin(&self) -> usize;

    fn get_end(&self) -> usize;

    fn get_rng(&self) -> Location {
        self.get_begin()..self.get_end()
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

    fn get_rng(&self) -> Location {
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
        Box<Expr>, //   condition
        TokLoc,    //             ?
        Box<Expr>, //               onTrue
        TokLoc,    //                      :
        Box<Expr>, //                        onFalse
    ),
}

impl Expr {}

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
            Expr::Ternary(b, _, _, _, _) => b.get_begin(),
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
            Expr::Ternary(_, _, _, _, e) => e.get_end(),
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            Expr::Atom(atm) => atm.get_rng(),
            Expr::FuncCall(call) => call.get_rng(),
            Expr::MethodCall(call) => call.get_rng(),
            Expr::Op(expr1, _, expr2) => expr1.get_begin()..expr2.get_end(),
            Expr::UnaryOp(op, expr) => op.get_begin()..expr.get_end(),
            Expr::PropertyAccess(prop_access) => prop_access.get_rng(),
            Expr::ParenExpr(begin, _, end) => *begin..*end,
            Expr::Indexed { base: b, end, .. } => b.get_begin()..*end,
            Expr::Ternary(b, _, _, _, e) => b.get_begin()..e.get_end(),
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

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul(TokLoc),
    Div(TokLoc),
    Add(TokLoc),
    Sub(TokLoc),
    Mod(TokLoc),
    And(TokLoc),
    Or(TokLoc),
    In(TokLoc),
    NotIn(TokLoc),
    Equal(TokLoc),
    G(TokLoc),
    L(TokLoc),
    GE(TokLoc),
    LE(TokLoc),
    NE(TokLoc),
    LBitshift(TokLoc),
    RBitshift(TokLoc),
}

impl Opcode {}

impl AstLoc for Opcode {
    fn get_begin(&self) -> usize {
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
            | Opcode::RBitshift(loc) => loc.get_begin(),
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

pub enum UnaryOpcode {
    Plus(TokLoc),
    Minus(TokLoc),
    Not(TokLoc),
    BitwiseNot(TokLoc),
}

impl UnaryOpcode {}

impl AstLoc for UnaryOpcode {
    fn get_begin(&self) -> usize {
        match self {
            UnaryOpcode::BitwiseNot(loc)
            | UnaryOpcode::Not(loc)
            | UnaryOpcode::Plus(loc)
            | UnaryOpcode::Minus(loc) => loc.get_begin(),
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

    fn get_rng(&self) -> Location {
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
    Atr(TokLoc),
    AddAtr(TokLoc),
    SubAtr(TokLoc),
    MulAtr(TokLoc),
    DivAtr(TokLoc),
    ModAtr(TokLoc),
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
