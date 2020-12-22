//! AST structures
use crate::grammar::lexer::{NumVal, Span};
use leafbuild_derive::Loc;
use std::fmt;
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

/// A spanned ast structure; holds data about where in the file a certain ast element is.
#[derive(PartialOrd, Eq, PartialEq, new)]
pub struct Spanned<T>(T, Span);

impl<T> fmt::Debug for Spanned<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Spanned")
            .field("data", &self.0)
            .field("location", &self.1)
            .finish()
    }
}

impl<T> Clone for Spanned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = source.0.clone();
        self.1 = source.1;
    }
}

impl<T> Loc for Spanned<T> {
    fn get_start(&self) -> usize {
        self.1.get_start()
    }

    fn get_end(&self) -> usize {
        self.1.get_end()
    }

    fn get_rng(&self) -> Location {
        self.1.get_rng()
    }
}

/// A small building block for expressions
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub enum Atom {
    /// A number
    Number(#[whole_span] Spanned<NumVal>),
    /// A bool
    Bool(#[whole_span] Spanned<bool>),
    /// A string
    Str(#[whole_span] Spanned<String>),
    /// An identifier
    Id(#[whole_span] Spanned<String>),
    /// A literal array
    ArrayLit(#[start_span] Span, Vec<Expr>, #[end_span] Span),
    /// A literal map
    MapLit(#[start_span] Span, Vec<NamedExpr>, #[end_span] Span),
}

/// An expression
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub enum Expr {
    /// A single atom
    Atom(#[whole_span] Atom),
    /// A binary operation
    Op(#[start_span] Box<Expr>, Opcode, #[end_span] Box<Expr>),
    /// A unary operation
    UnaryOp(#[start_span] UnaryOpcode, #[end_span] Box<Expr>),
    /// A function call
    FuncCall(#[whole_span] FuncCall),
    /// A method call
    MethodCall(#[whole_span] MethodCall),
    /// A property access operation
    PropertyAccess(#[whole_span] PropertyAccess),
    /// A `( Expr )` expression
    Paren {
        /// Span of left, opening parenthesis
        #[start_span]
        lparen: Span,
        /// Expression
        expr: Box<Expr>,
        /// Span of right, closing parenthesis
        #[end_span]
        rparen: Span,
    },
    /// An array/map indexing expression (`base [ index ]`)
    Indexed {
        /// The base expression
        #[start_span]
        base: Box<Expr>,
        /// The span of the left, opening bracket
        open_bracket: Span,
        /// The index expression
        index: Box<Expr>,
        /// The span of the right, closing bracket
        #[end_span]
        close_bracket: Span,
    },
    /// A ternary conditional expression
    Ternary {
        /// The condition expression
        #[start_span]
        condition: Box<Expr>,
        /// The span of the `?`
        qmark: Span,
        /// The expression tp evaluate if the condition is true
        if_true: Box<Expr>,
        /// The span of the `:`
        colon: Span,
        /// The expression to evaluate if the condition is false
        #[end_span]
        if_false: Box<Expr>,
    },
}

impl Expr {}

/// A property access expression
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct PropertyAccess {
    #[start_span]
    base: Box<Expr>,
    #[end_span]
    property_name: Spanned<String>,
}

impl PropertyAccess {
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

/// A binary operation
#[derive(Copy, Clone, Debug, Loc, PartialOrd, Eq, PartialEq, new)]
pub enum Opcode {
    /// `*`
    Mul(#[whole_span] Span),
    /// `/`
    Div(#[whole_span] Span),
    /// `+`
    Add(#[whole_span] Span),
    /// `-`
    Sub(#[whole_span] Span),
    /// `%`
    Mod(#[whole_span] Span),
    /// `and`
    And(#[whole_span] Span),
    /// `or`
    Or(#[whole_span] Span),
    /// `in`
    In(#[whole_span] Span),
    /// `not in`
    NotIn(#[whole_span] Span),
    /// `==`
    Equal(#[whole_span] Span),
    /// `>`
    G(#[whole_span] Span),
    /// `<`
    L(#[whole_span] Span),
    /// `>=`
    GE(#[whole_span] Span),
    /// `<=`
    LE(#[whole_span] Span),
    /// `!=`
    NE(#[whole_span] Span),
    /// `<<`
    LBitshift(#[whole_span] Span),
    /// `>>`
    RBitshift(#[whole_span] Span),
}

impl Opcode {}

/// An unary operation
#[derive(Debug, Copy, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub enum UnaryOpcode {
    /// `+`
    Plus(#[whole_span] Span),
    /// `-`
    Minus(#[whole_span] Span),
    /// `not`
    Not(#[whole_span] Span),
    /// `~`
    BitwiseNot(#[whole_span] Span),
}

impl UnaryOpcode {}

/// A function call
#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, new)]
pub struct FuncCall {
    func_name: Spanned<String>,
    left_paren: Span,
    func_args: FuncCallArgs,
    right_paren: Span,
}

impl FuncCall {
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
#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, new)]
pub struct FuncCallArgs {
    positional_args: Vec<PositionalArg>,
    named_args: Vec<NamedExpr>,
}

impl FuncCallArgs {
    /// Creates a new instance only with positional arguments
    #[must_use]
    pub fn new_only_positional(positional_args: Vec<PositionalArg>) -> Self {
        Self::new(positional_args, vec![])
    }

    /// Creates a new instance only with named arguments
    #[must_use]
    pub fn new_only_named(named_args: Vec<NamedExpr>) -> Self {
        Self::new(vec![], named_args)
    }

    /// Creates a new instance with no arguments(used where the function was called without any arguments)
    #[must_use]
    pub fn empty() -> Self {
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
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct PositionalArg {
    #[whole_span]
    value: Expr,
}

impl PositionalArg {
    pub(crate) const fn get_value(&self) -> &Expr {
        &self.value
    }
}

impl From<Expr> for PositionalArg {
    fn from(b: Expr) -> Self {
        Self { value: b }
    }
}

/// A named expression. Is created from `name = value`.
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct NamedExpr {
    #[start_span]
    name: Spanned<String>,
    eq_span: Span,
    #[end_span]
    value: Expr,
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

impl From<(Spanned<String>, Span, Expr)> for NamedExpr {
    fn from((name, eq_span, value): (Spanned<String>, Span, Expr)) -> Self {
        Self {
            name,
            eq_span,
            value,
        }
    }
}

/// A method call.
// TODO: refactor this
#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Loc, new)]
pub struct MethodCall {
    #[start_span]
    method_property: PropertyAccess,
    paren_open: Span,
    args: FuncCallArgs,
    #[end_span]
    paren_close: Span,
}

impl MethodCall {
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

/// An assignment
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct Assignment {
    #[start_span]
    bound_name: Expr,
    op: AtrOp,
    #[end_span]
    value: Expr,
}

impl Assignment {
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

/// A declaration
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct Declaration {
    #[start_span]
    let_tok: Span,
    name: Spanned<String>,
    eq: Span,
    #[end_span]
    value: Expr,
}

impl Declaration {
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

/// An assignment operation
#[derive(Debug, Copy, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub enum AtrOp {
    /// `=`
    Atr(#[whole_span] Span),
    /// `+=`
    AddAtr(#[whole_span] Span),
    /// `-=`
    SubAtr(#[whole_span] Span),
    /// `*=`
    MulAtr(#[whole_span] Span),
    /// `/=`
    DivAtr(#[whole_span] Span),
    /// `%=`
    ModAtr(#[whole_span] Span),
}

/// An `if expr { statements }` structure
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct If {
    #[start_span]
    if_tok: Span,
    condition: Expr,
    left_brace: Span,
    statements: Vec<Statement>,
    #[end_span]
    right_brace: Span,
}

impl If {
    #[must_use]
    pub(crate) const fn get_condition(&self) -> &Expr {
        &self.condition
    }

    #[must_use]
    pub(crate) const fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

/// An `else <if>` structure
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct ElseIf {
    #[start_span]
    else_tok: Span,
    #[end_span]
    if_: If,
}

impl ElseIf {
    #[must_use]
    pub(crate) const fn get_if(&self) -> &If {
        &self.if_
    }
}

/// An `else { statements }` structure
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct Else {
    #[start_span]
    else_tok: Span,
    left_brace: Span,
    statements: Vec<Statement>,
    #[end_span]
    right_brace: Span,
}

impl Else {
    #[must_use]
    pub(crate) const fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

/// A conditional statement
#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, new)]
pub struct ConditionalStatement {
    initial_if: If,
    else_ifs: Vec<ElseIf>,
    else_: Option<Else>,
}

impl ConditionalStatement {
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
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct RepetitiveStatement {
    #[start_span]
    foreach_tok: Span,
    for_in_expr: ForInExpr,
    left_brace: Span,
    statements: Vec<Statement>,
    #[end_span]
    right_brace: Span,
}

impl RepetitiveStatement {
    #[must_use]
    pub(crate) const fn get_for_in_expr(&self) -> &ForInExpr {
        &self.for_in_expr
    }
    #[must_use]
    pub(crate) const fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

/// The `name in expression` expression found in the foreach.
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct ForInExpr {
    #[start_span]
    name: Spanned<String>,
    in_tok: Span,
    #[end_span]
    expr: Expr,
}

impl ForInExpr {
    #[must_use]
    pub(crate) const fn get_name(&self) -> &Spanned<String> {
        &self.name
    }

    #[must_use]
    pub(crate) const fn get_expr(&self) -> &Expr {
        &self.expr
    }
}

/// A control statement.
#[derive(Debug, Copy, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub enum ControlStatement {
    /// `continue`
    Continue(#[whole_span] Span),
    /// `break`
    Break(#[whole_span] Span),
}

/// A statement.
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub enum Statement {
    /// Calls a function
    FuncCall(#[whole_span] FuncCall),
    /// Calls a method
    MethodCall(#[whole_span] MethodCall),
    /// Declares a variable
    Declaration(#[whole_span] Declaration),
    /// Assigns to a variable.
    Assignment(#[whole_span] Assignment),
    /// A conditional statement
    Conditional(#[whole_span] ConditionalStatement),
    /// A control statement
    Control(#[whole_span] ControlStatement),
    /// A repetitive statement
    Repetitive(#[whole_span] RepetitiveStatement),
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
#[derive(Debug, Clone, Loc, Eq, PartialEq, new)]
pub struct BuildDefinition {
    #[whole_span]
    statements: Vec<Statement>,
}

impl BuildDefinition {
    /// Returns a reference to the statements
    #[must_use]
    pub const fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}
