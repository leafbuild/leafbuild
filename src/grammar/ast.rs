//! AST structures
use crate::grammar::lexer::Span;
use std::any::Any;
use std::convert::TryInto;
use std::ops::Range;

type Location = Range<usize>;

/// Can get the location of something in a source file.
pub trait Loc {
    /// Returns the start index of this in the source file.
    fn get_start(&self) -> usize;

    /// Returns the end index of this in the source file.
    fn get_end(&self) -> usize;

    /// Returns the location of this(Range<usize>) in the source file.
    fn get_rng(&self) -> Location {
        self.get_start()..self.get_end()
    }
}

/// A number value
#[derive(Copy, Clone, Debug)]
pub enum NumVal {
    /// i32 number
    I32(i32),
    /// i64 number
    I64(i64),
    /// u32 number
    U32(u32),
    /// u64 number
    U64(u64),
}

macro_rules! add_digit_decl {
    ($name:ident, $coef:literal) => {
        const fn $name(self, digit: u8) -> Self {
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
            const fn into_unsigned(self) -> Self {
                match self {
                    Self::I32 => Self::U32,
                    Self::I64 => Self::U64,
                    x => x,
                }
            }
            const fn into_long(self) -> Self {
                match self {
                    Self::I32 => Self::I64,
                    Self::U32 => Self::U64,
                    x => x,
                }
            }
            const fn into_default_num_val(self) -> NumVal {
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
                    nmv.add_hex_digit(chr.to_digit(16).unwrap().try_into().unwrap())
                })
        } else if s.starts_with("0b") {
            s.chars()
                .skip(2)
                .take_while(|chr| chr.is_digit(2))
                .fold(tp.into_default_num_val(), |nmv, chr| {
                    nmv.add_bin_digit(chr.to_digit(2).unwrap().try_into().unwrap())
                })
        } else if s.starts_with('0') {
            s.chars()
                .skip(1)
                .take_while(|chr| chr.is_digit(8))
                .fold(tp.into_default_num_val(), |nmv, chr| {
                    nmv.add_oct_digit(chr.to_digit(8).unwrap().try_into().unwrap())
                })
        } else {
            s.chars()
                .take_while(|chr| chr.is_digit(10))
                .fold(tp.into_default_num_val(), |nmv, chr| {
                    nmv.add_dec_digit(chr.to_digit(10).unwrap().try_into().unwrap())
                })
        }
    }
}

/// A small building block for expressions
#[derive(Debug, Clone)]
pub enum Atom {
    /// A number
    Number((NumVal, Span)),
    /// A bool
    Bool((bool, Span)),
    /// A string
    Str((String, Span)),
    /// An identifier
    Id((String, Span)),
    /// A literal array
    ArrayLit((Span, Vec<Expr>, Span)),
    /// A literal map
    MapLit((Span, Vec<NamedExpr>, Span)),
}

impl Loc for Atom {
    fn get_start(&self) -> usize {
        match self {
            Self::Bool((_, loc))
            | Self::Number((_, loc))
            | Self::Id((_, loc))
            | Self::Str((_, loc)) => loc.get_start(),
            Self::ArrayLit((lbrace, _, _)) | Self::MapLit((lbrace, _, _)) => lbrace.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Self::Bool((_, loc))
            | Self::Number((_, loc))
            | Self::Id((_, loc))
            | Self::Str((_, loc)) => loc.get_end(),
            Self::ArrayLit((_, _, rbrace)) | Self::MapLit((_, _, rbrace)) => rbrace.get_end(),
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            Self::Bool((_, loc))
            | Self::Number((_, loc))
            | Self::Id((_, loc))
            | Self::Str((_, loc)) => loc.as_rng(),
            Self::ArrayLit((left_brace, _, right_brace))
            | Self::MapLit((left_brace, _, right_brace)) => {
                left_brace.get_start()..right_brace.get_end()
            }
        }
    }
}

/// An expression
#[derive(Debug, Clone)]
pub enum Expr {
    /// A single atom
    Atom(Atom),
    /// A binary operation
    Op(Box<Expr>, Opcode, Box<Expr>),
    /// A unary operation
    UnaryOp(UnaryOpcode, Box<Expr>),
    /// A function call
    FuncCall(FuncCall),
    /// A method call
    MethodCall(MethodCall),
    /// A property access operation
    PropertyAccess(PropertyAccess),
    /// A `( Expr )` expression
    Paren {
        /// Span of left, opening parenthesis
        lparen: Span,
        /// Expression
        expr: Box<Expr>,
        /// Span of right, closing parenthesis
        rparen: Span,
    },
    /// An array/map indexing expression (`base [ index ]`)
    Indexed {
        /// The base expression
        base: Box<Expr>,
        /// The span of the left, opening bracket
        open_bracket: Span,
        /// The index expression
        index: Box<Expr>,
        /// The span of the right, closing bracket
        close_bracket: Span,
    },
    /// A ternary conditional expression
    Ternary {
        /// The condition expression
        condition: Box<Expr>,
        /// The span of the `?`
        qmark: Span,
        /// The expression tp evaluate if the condition is true
        if_true: Box<Expr>,
        /// The span of the `:`
        colon: Span,
        /// The expression to evaluate if the condition is false
        if_false: Box<Expr>,
    },
}

pub(crate) struct Spanned<T> {
    data: T,
    location: Span,
}

impl Expr {}

impl Loc for Expr {
    fn get_start(&self) -> usize {
        match self {
            Self::Atom(atm) => atm.get_start(),
            Self::FuncCall(call) => call.get_start(),
            Self::MethodCall(call) => call.get_start(),
            Self::Op(expr, _, _) => expr.get_start(),
            Self::PropertyAccess(prop_access) => prop_access.get_start(),
            Self::Paren { lparen, .. } => lparen.get_start(),
            Self::UnaryOp(op, _) => op.get_start(),
            Self::Indexed { base, .. } => base.get_start(),
            Self::Ternary { condition, .. } => condition.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Self::Atom(atm) => atm.get_end(),
            Self::FuncCall(call) => call.get_end(),
            Self::MethodCall(call) => call.get_end(),
            Self::PropertyAccess(prop_access) => prop_access.get_end(),
            Self::Paren { rparen, .. } => rparen.get_end(),
            Self::Indexed { close_bracket, .. } => close_bracket.get_end(),
            Self::Ternary { if_false: expr, .. }
            | Self::UnaryOp(_, expr)
            | Self::Op(_, _, expr) => expr.get_end(),
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            Self::Atom(atm) => atm.get_rng(),
            Self::FuncCall(call) => call.get_rng(),
            Self::MethodCall(call) => call.get_rng(),
            Self::Op(expr1, _, expr2) => expr1.get_start()..expr2.get_end(),
            Self::UnaryOp(op, expr) => op.get_start()..expr.get_end(),
            Self::PropertyAccess(prop_access) => prop_access.get_rng(),
            Self::Paren { lparen, rparen, .. } => lparen.get_start()..rparen.get_end(),
            Self::Indexed {
                base,
                close_bracket,
                ..
            } => base.get_start()..close_bracket.get_end(),
            Self::Ternary {
                condition,
                if_false,
                ..
            } => condition.get_start()..if_false.get_end(),
        }
    }
}

/// A property access expression
#[derive(Debug, Clone)]
pub struct PropertyAccess {
    base: Box<Expr>,
    property_name: (String, Span),
}

impl PropertyAccess {
    /// Constructs a new property access from the base expression and property name and span.
    #[must_use]
    pub fn new(base: Box<Expr>, property_name: (String, Span)) -> Self {
        Self {
            base,
            property_name,
        }
    }

    #[must_use]
    pub(crate) const fn get_base(&self) -> &Expr {
        &self.base
    }

    #[must_use]
    pub(crate) const fn get_property_name(&self) -> &String {
        &self.property_name.0
    }

    #[must_use]
    pub(crate) const fn get_property_name_loc(&self) -> &Span {
        &self.property_name.1
    }
}

impl Loc for PropertyAccess {
    fn get_start(&self) -> usize {
        self.base.get_start()
    }

    fn get_end(&self) -> usize {
        self.property_name.1.get_end()
    }
}

/// A binary operation
#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    /// `*`
    Mul(Span),
    /// `/`
    Div(Span),
    /// `+`
    Add(Span),
    /// `-`
    Sub(Span),
    /// `%`
    Mod(Span),
    /// `and`
    And(Span),
    /// `or`
    Or(Span),
    /// `in`
    In(Span),
    /// `not in`
    NotIn(Span),
    /// `==`
    Equal(Span),
    /// `>`
    G(Span),
    /// `<`
    L(Span),
    /// `>=`
    GE(Span),
    /// `<=`
    LE(Span),
    /// `!=`
    NE(Span),
    /// `<<`
    LBitshift(Span),
    /// `>>`
    RBitshift(Span),
}

impl Opcode {}

impl Loc for Opcode {
    fn get_start(&self) -> usize {
        match self {
            Self::Mul(loc)
            | Self::Div(loc)
            | Self::Add(loc)
            | Self::Sub(loc)
            | Self::Mod(loc)
            | Self::And(loc)
            | Self::Or(loc)
            | Self::In(loc)
            | Self::NotIn(loc)
            | Self::Equal(loc)
            | Self::G(loc)
            | Self::L(loc)
            | Self::GE(loc)
            | Self::LE(loc)
            | Self::NE(loc)
            | Self::LBitshift(loc)
            | Self::RBitshift(loc) => loc.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Self::Mul(loc)
            | Self::Div(loc)
            | Self::Add(loc)
            | Self::Sub(loc)
            | Self::Mod(loc)
            | Self::And(loc)
            | Self::Or(loc)
            | Self::In(loc)
            | Self::NotIn(loc)
            | Self::Equal(loc)
            | Self::G(loc)
            | Self::L(loc)
            | Self::GE(loc)
            | Self::LE(loc)
            | Self::NE(loc)
            | Self::LBitshift(loc)
            | Self::RBitshift(loc) => loc.get_end(),
        }
    }
}

/// An unary operation
#[derive(Debug, Copy, Clone)]
pub enum UnaryOpcode {
    /// `+`
    Plus(Span),
    /// `-`
    Minus(Span),
    /// `not`
    Not(Span),
    /// `~`
    BitwiseNot(Span),
}

impl UnaryOpcode {}

impl Loc for UnaryOpcode {
    fn get_start(&self) -> usize {
        match self {
            Self::BitwiseNot(loc) | Self::Not(loc) | Self::Plus(loc) | Self::Minus(loc) => {
                loc.get_start()
            }
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Self::BitwiseNot(loc) | Self::Not(loc) | Self::Plus(loc) | Self::Minus(loc) => {
                loc.get_end()
            }
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            Self::BitwiseNot(loc) | Self::Not(loc) | Self::Plus(loc) | Self::Minus(loc) => {
                loc.as_rng()
            }
        }
    }
}

/// A function call
#[derive(Debug, Clone)]
pub struct FuncCall {
    func_name: (String, Span),
    func_args: FuncCallArgs,
    left_paren: Span,
    right_paren: Span,
}

impl FuncCall {
    /// Returns a new function call from the function name, spans of the parentheses and the arguments.
    #[must_use]
    pub const fn new(
        name: (String, Span),
        left_paren: Span,
        call_args: FuncCallArgs,
        right_paren: Span,
    ) -> Self {
        Self {
            func_name: name,
            func_args: call_args,
            left_paren,
            right_paren,
        }
    }

    /// Returns a reference to the name of the function this calls.
    #[must_use]
    pub const fn get_name(&self) -> &String {
        &self.func_name.0
    }

    /// Returns a reference to the span of the function name in the file
    #[must_use]
    pub const fn get_name_loc(&self) -> &Span {
        &self.func_name.1
    }

    /// Returns the function arguments
    #[must_use]
    pub const fn get_args(&self) -> &FuncCallArgs {
        &self.func_args
    }
}

impl Loc for FuncCall {
    fn get_start(&self) -> usize {
        self.func_name.1.get_start()
    }

    fn get_end(&self) -> usize {
        self.right_paren.get_end()
    }
}

/// The arguments passed to a function / method in a function / method call
#[derive(Debug, Clone)]
pub struct FuncCallArgs {
    positional_args: Vec<PositionalArg>,
    named_args: Vec<NamedExpr>,
}

impl FuncCallArgs {
    /// Constructor
    #[must_use]
    pub const fn new(positional_args: Vec<PositionalArg>, named_args: Vec<NamedExpr>) -> Self {
        Self {
            positional_args,
            named_args,
        }
    }

    /// Creates a new instance only with positional arguments
    #[must_use]
    pub const fn new_only_positional(positional_args: Vec<PositionalArg>) -> Self {
        Self::new(positional_args, vec![])
    }

    /// Creates a new instance only with named arguments
    #[must_use]
    pub const fn new_only_named(named_args: Vec<NamedExpr>) -> Self {
        Self::new(vec![], named_args)
    }

    /// Creates a new instance with no arguments(used where the function was called without any arguments)
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(vec![], vec![])
    }

    /// Returns a reference to the vector of positional arguments
    #[must_use]
    pub const fn get_positional_args(&self) -> &Vec<PositionalArg> {
        &self.positional_args
    }

    /// Returns a reference to the vector of named arguments
    #[must_use]
    pub const fn get_named_args(&self) -> &Vec<NamedExpr> {
        &self.named_args
    }
}

/// A positional argument
#[derive(Debug, Clone)]
pub struct PositionalArg {
    value: Expr,
}

impl PositionalArg {
    pub(crate) const fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl Loc for PositionalArg {
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

impl From<Box<Expr>> for PositionalArg {
    fn from(b: Box<Expr>) -> Self {
        Self { value: *b }
    }
}

/// A named expression. Is created from `name = value`.
#[derive(Debug, Clone)]
pub struct NamedExpr {
    name: (String, Span),
    eq_span: Span,
    value: Box<Expr>,
}

impl NamedExpr {
    /// Returns a reference to the name.
    #[must_use]
    pub const fn get_name(&self) -> &String {
        &self.name.0
    }

    /// Returns a reference to the value expression.
    #[must_use]
    pub const fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl Loc for NamedExpr {
    fn get_start(&self) -> usize {
        self.name.1.get_start()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
    }
}

impl From<((String, Span), Span, Box<Expr>)> for NamedExpr {
    fn from((name, eq_span, value): ((String, Span), Span, Box<Expr>)) -> Self {
        Self {
            name,
            eq_span,
            value,
        }
    }
}

/// A method call.
// TODO: refactor this
#[derive(Debug, Clone)]
pub struct MethodCall {
    method_property: PropertyAccess,
    args: FuncCallArgs,
    end_pos: usize,
}

impl MethodCall {
    /// Creates a method call from the method property, args and end position
    // TODO: refactor this
    #[must_use]
    pub const fn new(method_property: PropertyAccess, args: FuncCallArgs, end_pos: usize) -> Self {
        Self {
            method_property,
            args,
            end_pos,
        }
    }

    #[must_use]
    pub(crate) const fn get_base_expr(&self) -> &Expr {
        &self.method_property.base
    }

    #[must_use]
    pub(crate) const fn get_name(&self) -> &String {
        self.method_property.get_property_name()
    }

    #[must_use]
    pub(crate) const fn get_name_loc(&self) -> &Span {
        self.method_property.get_property_name_loc()
    }

    #[must_use]
    pub(crate) const fn get_args(&self) -> &FuncCallArgs {
        &self.args
    }
}

impl Loc for MethodCall {
    fn get_start(&self) -> usize {
        self.method_property.get_start()
    }

    fn get_end(&self) -> usize {
        self.end_pos
    }
}

/// An assignment
#[derive(Debug, Clone)]
pub struct Assignment {
    bound_name: Box<Expr>,
    op: AtrOp,
    value: Box<Expr>,
}

impl Assignment {
    /// Returns an assignment from the name, operation and value.
    #[must_use]
    pub const fn new(bound_name: Box<Expr>, op: AtrOp, value: Box<Expr>) -> Self {
        Self {
            bound_name,
            op,
            value,
        }
    }

    /// Returns the bound name expression
    #[must_use]
    pub const fn get_bound(&self) -> &Expr {
        &self.bound_name
    }

    /// Returns the operation
    #[must_use]
    pub const fn get_op(&self) -> &AtrOp {
        &self.op
    }

    /// Returns the value expression
    #[must_use]
    pub const fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl Loc for Assignment {
    fn get_start(&self) -> usize {
        self.bound_name.get_start()
    }

    fn get_end(&self) -> usize {
        self.value.get_end()
    }
}

/// A declaration
#[derive(Debug, Clone)]
pub struct Declaration {
    let_tok: Span,
    name: (String, Span),
    eq: Span,
    value: Box<Expr>,
}

impl Declaration {
    /// Constructor
    #[must_use]
    pub const fn new(let_tok: Span, name: (String, Span), eq: Span, value: Box<Expr>) -> Self {
        Self {
            let_tok,
            name,
            eq,
            value,
        }
    }

    /// Returns a reference to the name
    #[must_use]
    pub const fn get_name(&self) -> &String {
        &self.name.0
    }

    /// Returns a reference to the name span
    #[must_use]
    pub const fn get_name_loc(&self) -> &Span {
        &self.name.1
    }

    /// Returns a reference to the value expression
    #[must_use]
    pub const fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl Loc for Declaration {
    #[must_use]
    fn get_start(&self) -> usize {
        self.let_tok.get_start()
    }

    #[must_use]
    fn get_end(&self) -> usize {
        self.value.get_end()
    }
}

/// An assignment operation
#[derive(Debug, Copy, Clone)]
pub enum AtrOp {
    /// `=`
    Atr(Span),
    /// `+=`
    AddAtr(Span),
    /// `-=`
    SubAtr(Span),
    /// `*=`
    MulAtr(Span),
    /// `/=`
    DivAtr(Span),
    /// `%=`
    ModAtr(Span),
}

/// An `if expr { statements }` structure
#[derive(Debug, Clone)]
pub struct If {
    if_tok: Span,
    condition: Box<Expr>,
    left_brace: Span,
    statements: Vec<Statement>,
    right_brace: Span,
}

impl If {
    pub(crate) fn new(
        if_tok: Span,
        condition: Box<Expr>,
        left_brace: Span,
        statements: Vec<Statement>,
        right_brace: Span,
    ) -> Self {
        Self {
            if_tok,
            condition,
            left_brace,
            statements,
            right_brace,
        }
    }

    #[must_use]
    pub(crate) const fn get_condition(&self) -> &Expr {
        &self.condition
    }

    #[must_use]
    pub(crate) const fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

impl Loc for If {
    fn get_start(&self) -> usize {
        self.if_tok.get_start()
    }

    fn get_end(&self) -> usize {
        self.right_brace.get_end()
    }
}

/// An `else <if>` structure
#[derive(Debug, Clone)]
pub struct ElseIf {
    else_tok: Span,
    if_: If,
}

impl Loc for ElseIf {
    fn get_start(&self) -> usize {
        self.else_tok.get_start()
    }

    fn get_end(&self) -> usize {
        self.if_.get_end()
    }
}

impl ElseIf {
    #[must_use]
    pub(crate) const fn new(else_tok: Span, if_: If) -> Self {
        Self { else_tok, if_ }
    }

    #[must_use]
    pub(crate) const fn get_if(&self) -> &If {
        &self.if_
    }
}

/// An `else { statements }` structure
#[derive(Debug, Clone)]
pub struct Else {
    else_tok: Span,
    left_brace: Span,
    statements: Vec<Statement>,
    right_brace: Span,
}

impl Loc for Else {
    fn get_start(&self) -> usize {
        self.else_tok.get_start()
    }

    fn get_end(&self) -> usize {
        self.right_brace.get_end()
    }
}

impl Else {
    pub(crate) fn new(
        else_tok: Span,
        left_brace: Span,
        statements: Vec<Statement>,
        right_brace: Span,
    ) -> Self {
        Self {
            else_tok,
            left_brace,
            statements,
            right_brace,
        }
    }

    #[must_use]
    pub(crate) const fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

/// A conditional statement
#[derive(Debug, Clone)]
pub struct ConditionalStatement {
    initial_if: If,
    else_ifs: Vec<ElseIf>,
    else_: Option<Else>,
}

impl ConditionalStatement {
    pub(crate) fn new(initial_if: If, else_ifs: Vec<ElseIf>, else_: Option<Else>) -> Self {
        Self {
            initial_if,
            else_ifs,
            else_,
        }
    }

    #[must_use]
    pub(crate) const fn get_initial_if(&self) -> &If {
        &self.initial_if
    }

    #[must_use]
    pub(crate) const fn get_else_ifs(&self) -> &Vec<ElseIf> {
        &self.else_ifs
    }

    #[must_use]
    pub(crate) const fn get_else(&self) -> &Option<Else> {
        &self.else_
    }
}

impl Loc for ConditionalStatement {
    fn get_start(&self) -> usize {
        self.initial_if.get_start()
    }

    fn get_end(&self) -> usize {
        match &self.else_ {
            Some(els) => els.get_end(),
            None => match self.else_ifs.last() {
                Some(else_if) => else_if.get_end(),
                None => self.initial_if.get_end(),
            },
        }
    }
}

/// A repetitive statement (`foreach`)
#[derive(Debug, Clone)]
pub struct RepetitiveStatement {
    foreach_tok: Span,
    for_in_expr: ForInExpr,
    left_brace: Span,
    statements: Vec<Statement>,
    right_brace: Span,
}

impl RepetitiveStatement {
    /// Constructor
    #[must_use]
    pub const fn new(
        foreach_tok: Span,
        for_in_expr: ForInExpr,
        left_brace: Span,
        statements: Vec<Statement>,
        right_brace: Span,
    ) -> Self {
        Self {
            foreach_tok,
            for_in_expr,
            left_brace,
            statements,
            right_brace,
        }
    }

    #[must_use]
    pub(crate) const fn get_for_in_expr(&self) -> &ForInExpr {
        &self.for_in_expr
    }
    #[must_use]
    pub(crate) const fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

impl Loc for RepetitiveStatement {
    fn get_start(&self) -> usize {
        self.for_in_expr.get_start()
    }

    fn get_end(&self) -> usize {
        self.right_brace.get_end()
    }
}

/// The `name in expression` expression found in the foreach.
#[derive(Debug, Clone)]
pub struct ForInExpr {
    name: (String, Span),
    in_tok: Span,
    expr: Box<Expr>,
}

impl ForInExpr {
    /// Constructor
    #[must_use]
    pub const fn new(name: (String, Span), in_tok: Span, expr: Box<Expr>) -> Self {
        Self { name, in_tok, expr }
    }

    #[must_use]
    pub(crate) const fn get_name(&self) -> &(String, Span) {
        &self.name
    }

    #[must_use]
    pub(crate) const fn get_expr(&self) -> &Expr {
        &self.expr
    }
}

impl Loc for ForInExpr {
    fn get_start(&self) -> usize {
        self.name.1.get_start()
    }

    fn get_end(&self) -> usize {
        self.expr.get_end()
    }
}

/// A control statement.
#[derive(Debug, Copy, Clone)]
pub enum ControlStatement {
    /// `continue`
    Continue(Span),
    /// `break`
    Break(Span),
}

impl Loc for ControlStatement {
    fn get_start(&self) -> usize {
        match self {
            Self::Continue(loc) | Self::Break(loc) => loc.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Self::Continue(loc) | Self::Break(loc) => loc.get_end(),
        }
    }

    fn get_rng(&self) -> Location {
        match self {
            Self::Continue(loc) | Self::Break(loc) => loc.as_rng(),
        }
    }
}

/// A statement.
#[derive(Debug, Clone)]
pub enum Statement {
    /// Calls a function
    FuncCall(FuncCall),
    /// Calls a method
    MethodCall(MethodCall),
    /// Declares a variable
    Declaration(Declaration),
    /// Assigns to a variable.
    Assignment(Assignment),
    /// A conditional statement
    Conditional(ConditionalStatement),
    /// A control statement
    Control(ControlStatement),
    /// A repetitive statement
    Repetitive(RepetitiveStatement),
}

impl Loc for Statement {
    fn get_start(&self) -> usize {
        match self {
            Self::FuncCall(call) => call.get_start(),
            Self::MethodCall(call) => call.get_start(),
            Self::Declaration(decl) => decl.get_start(),
            Self::Assignment(assignment) => assignment.get_start(),
            Self::Control(control_statement) => control_statement.get_start(),
            Self::Conditional(conditional_statement) => conditional_statement.get_start(),
            Self::Repetitive(repetitive) => repetitive.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Self::FuncCall(call) => call.get_end(),
            Self::MethodCall(call) => call.get_end(),
            Self::Declaration(decl) => decl.get_end(),
            Self::Assignment(assignment) => assignment.get_end(),
            Self::Control(control_statement) => control_statement.get_end(),
            Self::Conditional(conditional_statement) => conditional_statement.get_end(),
            Self::Repetitive(repetitive) => repetitive.get_end(),
        }
    }
}

impl<T> Loc for Vec<T>
where
    T: Loc,
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

impl<T> Loc for &[T]
where
    T: Loc,
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

/// The whole build definition
#[derive(Debug, Clone)]
pub struct BuildDefinition {
    statements: Vec<Statement>,
}

impl BuildDefinition {
    /// Constructor
    #[must_use]
    pub const fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    /// Returns a reference to the statements
    #[must_use]
    pub const fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}
