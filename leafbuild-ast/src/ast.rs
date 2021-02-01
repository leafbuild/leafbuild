//! AST structures
use crate::span::Span;
use crate::token_data::NumVal;
use std::fmt;
use std::ops::{Deref, DerefMut, Range};

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
#[derive(PartialOrd, Eq, PartialEq, Loc, new)]
pub struct Spanned<T>(pub T, #[whole_span] pub Span);

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for Spanned<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

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
    FnCall(#[whole_span] FnCall),
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
        /// The expression to evaluate if the condition is true
        if_true: Box<Expr>,
        /// The span of the `:`
        colon: Span,
        /// The expression to evaluate if the condition is false
        #[end_span]
        if_false: Box<Expr>,
    },
}

/// A property access expression
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct PropertyAccess {
    /// The base expression
    /// ```text
    /// object . property_name
    /// ^^^^^^
    /// ```
    #[start_span]
    pub base: Box<Expr>,
    /// The span of the `.` token
    pub dot_span: Span,
    /// The name of the property and the associated span
    #[end_span]
    pub property_name: Spanned<String>,
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
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct FnCall {
    /// The base expression we are calling
    #[start_span]
    pub func_base: Box<Expr>,
    /// The span of the `(` token
    pub left_paren: Span,
    /// The arguments
    pub func_args: FnCallArgs,
    /// The span of the `)` token
    #[end_span]
    pub right_paren: Span,
}

/// The arguments passed to a function / method in a function / method call
#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, new)]
pub struct FnCallArgs {
    /// The positional arguments
    pub positional_args: Vec<PositionalArg>,
    /// The named arguments
    pub named_args: Vec<NamedExpr>,
}

impl From<(Vec<PositionalArg>, Vec<NamedExpr>)> for FnCallArgs {
    fn from((positional_args, named_args): (Vec<PositionalArg>, Vec<NamedExpr>)) -> Self {
        Self {
            positional_args,
            named_args,
        }
    }
}

/// A positional argument
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct PositionalArg(#[whole_span] pub Expr);

impl From<Expr> for PositionalArg {
    fn from(expr: Expr) -> Self {
        Self(expr)
    }
}

/// A named expression. Is created from `name = value`.
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct NamedExpr {
    /// The name, along with the associated span.
    #[start_span]
    pub name: Spanned<String>,
    /// The span of the `=` token
    pub eq_span: Span,
    /// The value of the expression
    #[end_span]
    pub value: Expr,
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
/// Goes like
/// ```text
/// base_expression . method_name ( arguments )
/// ```
/// Where `base_expression . method_name` make a [`PropertyAccess`] expression.
#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Loc, new)]
pub struct MethodCall {
    /// The method property this method call references
    #[start_span]
    pub method_property: PropertyAccess,
    /// The span of the `(` token
    pub paren_open: Span,
    /// The arguments
    pub args: FnCallArgs,
    /// The span of the `)` token
    #[end_span]
    pub paren_close: Span,
}

/// An assignment
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct Assignment {
    /// The expression to get a mutable reference from and use with the `AtrOp`
    #[start_span]
    pub bound_name: Expr,
    /// The attribution operation: `=`, `+=`, `-=`...
    pub op: AtrOp,
    /// The expression of the value to assign
    #[end_span]
    pub value: Expr,
}

/// A declaration
/// Goes like
/// ```text
/// let name = value
/// ```
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct Declaration {
    /// The span of the `let` token
    #[start_span]
    pub let_tok: Span,
    /// The name of the variable it is declaring
    pub name: Spanned<String>,
    /// The span of the `=` token
    pub eq: Span,
    /// The expression of the value to assign
    #[end_span]
    pub value: Expr,
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
    /// The span of the `if` token
    #[start_span]
    pub if_tok: Span,
    /// The condition expression
    pub condition: Expr,
    /// The span of the `{` token
    pub left_brace: Span,
    /// The statements
    pub statements: Vec<Statement>,
    /// The span of the `}` token
    #[end_span]
    pub right_brace: Span,
}

/// An `else <if>` structure
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct ElseIf {
    /// The span of the `else` token
    #[start_span]
    pub else_tok: Span,
    /// The underlying `If`
    #[end_span]
    pub if_: If,
}

/// An `else { statements }` structure
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct Else {
    /// The span of the `else` token
    #[start_span]
    pub else_tok: Span,
    /// The span of the `{` token
    pub left_brace: Span,
    /// The statements
    pub statements: Vec<Statement>,
    /// The span of the `}` token
    #[end_span]
    pub right_brace: Span,
}

/// A conditional statement
#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, new)]
pub struct ConditionalStatement {
    /// The first `if`
    pub initial_if: If,
    /// All `else if`s
    pub else_ifs: Vec<ElseIf>,
    /// The `else`
    pub else_: Option<Else>,
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
    /// The span of the `foreach` token
    #[start_span]
    pub foreach_tok: Span,
    /// The `... in ...` expression
    pub for_in_expr: ForInExpr,
    /// The span of the `{` token
    pub left_brace: Span,
    /// The statements
    pub statements: Vec<Statement>,
    /// The span of the `}` token
    #[end_span]
    pub right_brace: Span,
}

/// The `name in expression` expression found in the foreach.
#[derive(Debug, Clone, Loc, PartialOrd, Eq, PartialEq, new)]
pub struct ForInExpr {
    /// The name of the variable used to iterate
    #[start_span]
    pub name: Spanned<String>,
    /// The span of the `in` token
    pub in_tok: Span,
    /// The expression to iterate over
    #[end_span]
    pub expr: Expr,
}

/// A control statement.
#[derive(Clone, Debug, PartialOrd, Eq, PartialEq)]
pub enum ControlStatement {
    /// `continue`
    Continue(Span),
    /// `break`
    Break(Span),
    /// `return ...`
    Return(Span, Option<Expr>),
}

impl Loc for ControlStatement {
    fn get_start(&self) -> usize {
        match self {
            Self::Continue(s) | Self::Break(s) | Self::Return(s, ..) => s.get_start(),
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Self::Return(s, None) | Self::Continue(s) | Self::Break(s) => s.get_end(),
            Self::Return(_, Some(expr)) => expr.get_end(),
        }
    }
}

/// The body of a function
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct FnBody {
    /// The span of the `{` token
    #[start_span]
    pub left_brace: Span,
    /// The statements of this function
    pub statements: Vec<Statement>,
    /// The tail return expression, similar to rust's
    pub tail_expr: Option<Expr>,
    /// The span of the `}` token
    #[end_span]
    pub right_brace: Span,
}

impl From<(Span, Vec<Statement>, Option<Expr>, Span)> for FnBody {
    fn from(
        (left_brace, statements, tail_expr, right_brace): (
            Span,
            Vec<Statement>,
            Option<Expr>,
            Span,
        ),
    ) -> Self {
        Self {
            left_brace,
            statements,
            tail_expr,
            right_brace,
        }
    }
}

/// A type ref
#[derive(Clone, Debug, Loc, Eq, PartialEq)]
pub enum TypeRef {
    /// A type referred like `type_name`, such as `int`, `float`, `Project`
    Named {
        /// The name of the type
        #[whole_span]
        ty_name: Spanned<String>,
    },
    /// A type referred like `type_name<generic_1, generic_2, ...>`
    GenericNamed {
        /// The name of the base type
        #[start_span]
        ty_name: Spanned<String>,
        /// The generic types
        #[end_span]
        generics: TypeRefGenerics,
    },
    /// Function type
    Fn {
        /// The span of the `fn` token
        #[start_span]
        fn_span: Span,
        /// The span of the `(` token
        left_paren: Span,

        /// The types
        tys: Vec<TypeRef>,

        /// The span of the `)` token
        #[end_span]
        right_paren: Span,
    },
}

impl From<Spanned<String>> for TypeRef {
    fn from(ty_name: Spanned<String>) -> Self {
        Self::Named { ty_name }
    }
}

/// The type ref generics
#[derive(Clone, Debug, Loc, Eq, PartialEq)]
pub struct TypeRefGenerics {
    /// The span of the `<` token
    #[start_span]
    pub less_than: Span,

    /// The list of generic types
    pub tys: Vec<TypeRef>,

    /// The span of the `>` token
    #[end_span]
    pub greater_than: Span,
}

/// A positional parameter
#[derive(Clone, Debug, Loc, Eq, PartialEq)]
pub struct PositionalParam {
    /// The name of the param
    #[start_span]
    pub name: Spanned<String>,
    /// The span of the `:` token
    pub colon: Span,
    /// The type of the param
    #[end_span]
    pub ty: TypeRef,
}

impl From<(Spanned<String>, Span, TypeRef)> for PositionalParam {
    fn from((name, colon, ty): (Spanned<String>, Span, TypeRef)) -> Self {
        Self { name, colon, ty }
    }
}

/// A parameter with a default value
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultParam {
    /// The name of the param
    pub name: Spanned<String>,
    /// The span of the `:` token
    pub colon: Span,
    /// The type of the param
    pub ty: TypeRef,
    /// The span of the `=` token
    pub eq: Span,
    /// The expression to evaluate for the default value
    pub value: Expr,
}

impl From<(Spanned<String>, Span, TypeRef, Span, Expr)> for DefaultParam {
    fn from((name, colon, ty, eq, value): (Spanned<String>, Span, TypeRef, Span, Expr)) -> Self {
        Self {
            name,
            colon,
            ty,
            eq,
            value,
        }
    }
}

/// A function declaration
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct FnDecl {
    /// The span of the `fn` token
    #[start_span]
    pub fn_span: Span,
    /// The name of the function
    pub name: Spanned<String>,

    /// The span of the `(` token
    pub left_paren: Span,

    /// The positional parameters
    pub positional_params: Vec<PositionalParam>,
    /// The default parameters
    pub default_params: Vec<DefaultParam>,

    /// The span of the `)` token
    pub right_paren: Span,

    /// The function body
    #[end_span]
    pub body: FnBody,
}

type FnDeclTuple = (
    Span,
    Spanned<String>,
    Span,
    (Vec<PositionalParam>, Vec<DefaultParam>),
    Span,
    FnBody,
);

impl From<FnDeclTuple> for FnDecl {
    fn from(
        (fn_span, name, left_paren, (positional_params, default_params), right_paren, body): FnDeclTuple,
    ) -> Self {
        Self {
            fn_span,
            name,
            left_paren,
            positional_params,
            default_params,
            right_paren,
            body,
        }
    }
}

/// A language item
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub enum LangItem {
    /// A function declaration
    FnDecl(#[whole_span] FnDecl),

    /// Statement
    Statement(#[whole_span] Statement),
}

/// A statement.
#[derive(Clone, Debug, Loc, PartialOrd, Eq, PartialEq, new)]
pub enum Statement {
    /// Executes an expression, calling functions and methods that may or may not have side effects.
    ExecExpr(#[whole_span] Expr),
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
    /// The items
    #[whole_span]
    pub items: Vec<LangItem>,
}
