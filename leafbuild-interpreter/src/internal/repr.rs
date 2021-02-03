//! Intermediate representation of the parse tree.
//! There are three types of ADTs defined here, roughly
//! matched by:
//! - `Ir*Data`, the ir data that we add to the node, like types.
//! They *should* implement [`Default`], and that default value
//! should be used when we only have the data from the parse tree.
//! Note that [`<Ty as Default>::default()`] = [`TyUnknown`].
//! - `IrAst*`, made to reflect the types in [`leafbuild_ast::ast`].
//! - `Ir*`, which glues the other two together
use crate::internal::values::types::*;
use itertools::Itertools;
use leafbuild_ast::ast::*;
use leafbuild_ast::leafbuild_derive::Loc;
use leafbuild_ast::token_data::NumVal;
use leafbuild_ast::Span;
use std::ops::{Deref, DerefMut};

macro_rules! impl_deref {
    ($name:ident, $deref_target:ident) => {
        impl Deref for $name {
            type Target = $deref_target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

//
//  Atom
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAtom(#[whole_span] pub IrAstAtom, pub IrAtomData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrAtomData(pub Ty);
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub enum IrAstAtom {
    Number(#[whole_span] Spanned<NumVal>),
    Bool(#[whole_span] Spanned<bool>),
    Str(#[whole_span] Spanned<String>),
    Id(#[whole_span] Spanned<String>),
    ArrayLit(#[start_span] Span, Vec<IrExpr>, #[end_span] Span),
    MapLit(#[start_span] Span, Vec<IrNamedExpr>, #[end_span] Span),
}

impl_deref! {IrAtom, IrAstAtom}

impl From<Atom> for IrAtom {
    fn from(atom: Atom) -> Self {
        Self::from((IrAstAtom::from(atom), IrAtomData::default()))
    }
}

impl From<(IrAstAtom, IrAtomData)> for IrAtom {
    fn from((ast, data): (IrAstAtom, IrAtomData)) -> Self {
        Self(ast, data)
    }
}

impl From<Atom> for IrAstAtom {
    fn from(atom: Atom) -> Self {
        match atom {
            Atom::Number(num) => Self::Number(num),
            Atom::Bool(bool) => Self::Bool(bool),
            Atom::Str(str) => Self::Str(str),
            Atom::Id(id) => Self::Id(id),
            Atom::ArrayLit(left_paren, values, right_paren) => Self::ArrayLit(
                left_paren,
                values.into_iter().map_into::<IrExpr>().collect(),
                right_paren,
            ),
            Atom::MapLit(left_paren, values, right_paren) => Self::MapLit(
                left_paren,
                values.into_iter().map_into::<IrNamedExpr>().collect(),
                right_paren,
            ),
        }
    }
}

//
// Expr
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrExpr(#[whole_span] pub IrAstExpr, pub IrExprData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrExprData(pub Ty);
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub enum IrAstExpr {
    Atom(#[whole_span] IrAtom),
    Op(#[whole_span] Box<IrExpr>, Opcode, #[end_span] Box<IrExpr>),
    UnaryOp(#[start_span] UnaryOpcode, #[end_span] Box<IrExpr>),
    FnCall(#[whole_span] IrFnCall),
    MethodCall(#[whole_span] Box<IrMethodCall>),
    PropertyAccess(#[whole_span] IrPropertyAccess),
    Paren {
        #[start_span]
        left_paren: Span,
        expr: Box<IrExpr>,
        #[end_span]
        right_paren: Span,
    },
    Indexed {
        #[start_span]
        base: Box<IrExpr>,
        left_bracket: Span,
        index: Box<IrExpr>,
        #[end_span]
        right_bracket: Span,
    },
    Ternary {
        #[start_span]
        condition: Box<IrExpr>,
        qmark: Span,
        if_true: Box<IrExpr>,
        colon: Span,
        #[end_span]
        if_false: Box<IrExpr>,
    },
}

impl_deref! {IrExpr, IrAstExpr}

impl From<Expr> for IrExpr {
    fn from(expr: Expr) -> Self {
        Self::from((IrAstExpr::from(expr), IrExprData::default()))
    }
}

impl From<(IrAstExpr, IrExprData)> for IrExpr {
    fn from((ast, data): (IrAstExpr, IrExprData)) -> Self {
        Self(ast, data)
    }
}

impl From<Expr> for IrAstExpr {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Atom(atom) => {
                Self::Atom(IrAtom::from((IrAstAtom::from(atom), IrAtomData::default())))
            }
            Expr::Op(left, op, right) => Self::Op(
                Box::new(IrExpr::from(*left)),
                op,
                Box::new(IrExpr::from(*right)),
            ),
            Expr::UnaryOp(op, expr) => Self::UnaryOp(op, Box::new(IrExpr::from(*expr))),
            Expr::FnCall(fn_call) => Self::FnCall(IrFnCall::from(fn_call)),
            Expr::MethodCall(method_call) => {
                Self::MethodCall(Box::new(IrMethodCall::from(method_call)))
            }
            Expr::PropertyAccess(property_access) => {
                Self::PropertyAccess(IrPropertyAccess::from(property_access))
            }
            Expr::Paren {
                left_paren,
                expr,
                right_paren,
            } => Self::Paren {
                left_paren,
                expr: Box::new(IrExpr::from(*expr)),
                right_paren,
            },
            Expr::Indexed {
                base,
                left_bracket,
                index,
                right_bracket,
            } => Self::Indexed {
                base: Box::new(IrExpr::from(*base)),
                left_bracket,
                index: Box::new(IrExpr::from(*index)),
                right_bracket,
            },
            Expr::Ternary {
                condition,
                qmark,
                if_true,
                colon,
                if_false,
            } => Self::Ternary {
                condition: Box::new(IrExpr::from(*condition)),
                qmark,
                if_true: Box::new(IrExpr::from(*if_true)),
                colon,
                if_false: Box::new(IrExpr::from(*if_false)),
            },
        }
    }
}

//
// Property access
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrPropertyAccess(
    #[whole_span] pub IrAstPropertyAccess,
    pub IrPropertyAccessData,
);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrPropertyAccessData {
    pub base_ty: Ty,
    pub property_ty: Ty,
}
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstPropertyAccess {
    #[start_span]
    pub base: Box<IrExpr>,
    pub dot_span: Span,
    #[end_span]
    pub property_name: Spanned<String>,
}

impl_deref! {IrPropertyAccess, IrAstPropertyAccess}

impl From<PropertyAccess> for IrPropertyAccess {
    fn from(property_access: PropertyAccess) -> Self {
        Self::from((
            IrAstPropertyAccess::from(property_access),
            IrPropertyAccessData::default(),
        ))
    }
}

impl From<(IrAstPropertyAccess, IrPropertyAccessData)> for IrPropertyAccess {
    fn from((ast, data): (IrAstPropertyAccess, IrPropertyAccessData)) -> Self {
        Self(ast, data)
    }
}

impl From<PropertyAccess> for IrAstPropertyAccess {
    fn from(
        PropertyAccess {
            base,
            dot_span,
            property_name,
        }: PropertyAccess,
    ) -> Self {
        Self {
            base: Box::new(IrExpr::from(*base)),
            dot_span,
            property_name,
        }
    }
}

//
// Function call
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrFnCall(#[whole_span] pub IrAstFnCall, pub IrFnCallData);
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct IrFnCallData(pub Ty);

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstFnCall {
    #[start_span]
    pub fn_base: Box<IrExpr>,
    pub left_paren: Span,

    pub fn_args: IrFnCallArgs,

    #[end_span]
    pub right_paren: Span,
}

impl_deref! {IrFnCall, IrAstFnCall}

impl From<FnCall> for IrFnCall {
    fn from(fn_call: FnCall) -> Self {
        Self::from((IrAstFnCall::from(fn_call), IrFnCallData::default()))
    }
}

impl From<(IrAstFnCall, IrFnCallData)> for IrFnCall {
    fn from((ast, data): (IrAstFnCall, IrFnCallData)) -> Self {
        Self(ast, data)
    }
}

impl From<FnCall> for IrAstFnCall {
    fn from(
        FnCall {
            fn_base,
            left_paren,
            fn_args,
            right_paren,
        }: FnCall,
    ) -> Self {
        Self {
            fn_base: Box::new(IrExpr::from(*fn_base)),
            left_paren,
            fn_args: IrFnCallArgs::from(fn_args),
            right_paren,
        }
    }
}

//
// Function call args
//
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IrFnCallArgs(pub IrAstFnCallArgs, pub IrFnCallArgsData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrFnCallArgsData {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IrAstFnCallArgs {
    pub positional_args: Vec<IrPositionalArg>,
    pub named_args: Vec<IrNamedExpr>,
}

impl_deref! {IrFnCallArgs, IrAstFnCallArgs}

impl From<FnCallArgs> for IrFnCallArgs {
    fn from(args: FnCallArgs) -> Self {
        Self::from((IrAstFnCallArgs::from(args), IrFnCallArgsData::default()))
    }
}

impl From<(IrAstFnCallArgs, IrFnCallArgsData)> for IrFnCallArgs {
    fn from((ast, data): (IrAstFnCallArgs, IrFnCallArgsData)) -> Self {
        Self(ast, data)
    }
}

impl From<FnCallArgs> for IrAstFnCallArgs {
    fn from(
        FnCallArgs {
            positional_args,
            named_args,
        }: FnCallArgs,
    ) -> Self {
        Self {
            positional_args: positional_args
                .into_iter()
                .map_into::<IrPositionalArg>()
                .collect(),
            named_args: named_args.into_iter().map_into::<IrNamedExpr>().collect(),
        }
    }
}

//
// Positional arg
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrPositionalArg(
    #[whole_span] pub IrAstPositionalArg,
    pub IrPositionalArgData,
);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrPositionalArgData;
#[derive(Loc, Debug, Clone, Eq, PartialEq)]
pub struct IrAstPositionalArg(#[whole_span] pub IrExpr);

impl_deref! {IrPositionalArg, IrAstPositionalArg}

impl From<PositionalArg> for IrPositionalArg {
    fn from(arg: PositionalArg) -> Self {
        Self::from((
            IrAstPositionalArg::from(arg),
            IrPositionalArgData::default(),
        ))
    }
}

impl From<(IrAstPositionalArg, IrPositionalArgData)> for IrPositionalArg {
    fn from((ast, data): (IrAstPositionalArg, IrPositionalArgData)) -> Self {
        Self(ast, data)
    }
}

impl From<PositionalArg> for IrAstPositionalArg {
    fn from(arg: PositionalArg) -> Self {
        Self(IrExpr::from(arg.0))
    }
}

//
// Named expressions
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrNamedExpr(#[whole_span] pub IrAstNamedExpr, pub IrNamedExprData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrNamedExprData();
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstNamedExpr {
    #[start_span]
    pub name: Spanned<String>,
    pub eq_span: Span,
    #[end_span]
    pub value: IrExpr,
}

impl_deref! {IrNamedExpr, IrAstNamedExpr}

impl From<NamedExpr> for IrNamedExpr {
    fn from(expr: NamedExpr) -> Self {
        Self::from((IrAstNamedExpr::from(expr), IrNamedExprData::default()))
    }
}

impl From<(IrAstNamedExpr, IrNamedExprData)> for IrNamedExpr {
    fn from((ast, data): (IrAstNamedExpr, IrNamedExprData)) -> Self {
        Self(ast, data)
    }
}

impl From<NamedExpr> for IrAstNamedExpr {
    fn from(
        NamedExpr {
            name,
            eq_span,
            value,
        }: NamedExpr,
    ) -> Self {
        Self {
            name,
            eq_span,
            value: IrExpr::from(value),
        }
    }
}

//
// Method call
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrMethodCall(#[whole_span] pub IrAstMethodCall, pub IrMethodCallData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrMethodCallData(Ty);
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstMethodCall {
    #[start_span]
    pub method_property: IrPropertyAccess,
    pub left_paren: Span,
    pub args: IrFnCallArgs,
    #[end_span]
    pub right_paren: Span,
}

impl_deref! {IrMethodCall, IrAstMethodCall}

impl From<MethodCall> for IrMethodCall {
    fn from(method_call: MethodCall) -> Self {
        Self::from((
            IrAstMethodCall::from(method_call),
            IrMethodCallData::default(),
        ))
    }
}

impl From<(IrAstMethodCall, IrMethodCallData)> for IrMethodCall {
    fn from((ast, data): (IrAstMethodCall, IrMethodCallData)) -> Self {
        Self(ast, data)
    }
}

impl From<MethodCall> for IrAstMethodCall {
    fn from(
        MethodCall {
            method_property,
            left_paren,
            args,
            right_paren,
        }: MethodCall,
    ) -> Self {
        Self {
            method_property: IrPropertyAccess::from(method_property),
            left_paren,
            args: IrFnCallArgs::from(args),
            right_paren,
        }
    }
}

//
// Assignment
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAssignment(#[whole_span] pub IrAstAssignment, pub IrAssignmentData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrAssignmentData(pub Ty);
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstAssignment {
    #[start_span]
    pub bound_expr: IrExpr,
    pub op: AtrOp,
    pub value: IrExpr,
    #[end_span]
    pub semicolon: Span,
}

impl_deref! {IrAssignment, IrAstAssignment}

impl From<Assignment> for IrAssignment {
    fn from(assignment: Assignment) -> Self {
        Self::from((
            IrAstAssignment::from(assignment),
            IrAssignmentData::default(),
        ))
    }
}

impl From<(IrAstAssignment, IrAssignmentData)> for IrAssignment {
    fn from((ast, data): (IrAstAssignment, IrAssignmentData)) -> Self {
        Self(ast, data)
    }
}

impl From<Assignment> for IrAstAssignment {
    fn from(
        Assignment {
            bound_expr,
            op,
            value,
            semicolon,
        }: Assignment,
    ) -> Self {
        Self {
            bound_expr: IrExpr::from(bound_expr),
            op,
            value: IrExpr::from(value),
            semicolon,
        }
    }
}

//
// Expression statement
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrExprStatement(
    #[whole_span] pub IrAstExprStatement,
    pub IrExprStatementData,
);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrExprStatementData(pub Ty);

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstExprStatement {
    #[start_span]
    pub expr: IrExpr,

    #[end_span]
    pub semicolon: Span,
}

impl_deref! {IrExprStatement, IrAstExprStatement}

impl From<ExprStatement> for IrExprStatement {
    fn from(expr_statement: ExprStatement) -> Self {
        Self::from((
            IrAstExprStatement::from(expr_statement),
            IrExprStatementData::default(),
        ))
    }
}

impl From<(IrAstExprStatement, IrExprStatementData)> for IrExprStatement {
    fn from((ast, data): (IrAstExprStatement, IrExprStatementData)) -> Self {
        Self(ast, data)
    }
}

impl From<ExprStatement> for IrAstExprStatement {
    fn from(ExprStatement { expr, semicolon }: ExprStatement) -> Self {
        Self {
            expr: IrExpr::from(expr),
            semicolon,
        }
    }
}

//
// Declaration
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrDeclaration(#[whole_span] pub IrAstDeclaration, pub IrDeclarationData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrDeclarationData(pub Ty);

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstDeclaration {
    #[start_span]
    pub let_tok: Span,
    pub name: Spanned<String>,
    pub eq: Span,
    pub value: IrExpr,
    #[end_span]
    pub semicolon: Span,
}

impl_deref! {IrDeclaration, IrAstDeclaration}

impl From<Declaration> for IrDeclaration {
    fn from(declaration: Declaration) -> Self {
        Self::from((
            IrAstDeclaration::from(declaration),
            IrDeclarationData::default(),
        ))
    }
}

impl From<(IrAstDeclaration, IrDeclarationData)> for IrDeclaration {
    fn from((ast, data): (IrAstDeclaration, IrDeclarationData)) -> Self {
        Self(ast, data)
    }
}

impl From<Declaration> for IrAstDeclaration {
    fn from(
        Declaration {
            let_tok,
            name,
            eq,
            value,
            semicolon,
        }: Declaration,
    ) -> Self {
        Self {
            let_tok,
            name,
            eq,
            value: IrExpr::from(value),
            semicolon,
        }
    }
}

//
// If
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrIf(#[whole_span] pub IrAstIf, pub IrIfData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrIfData(pub Ty);

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstIf {
    #[start_span]
    pub if_tok: Span,
    pub condition: IrExpr,
    pub left_brace: Span,
    pub statements: Vec<IrStatement>,
    #[end_span]
    pub right_brace: Span,
}

impl_deref! {IrIf, IrAstIf}

impl From<If> for IrIf {
    fn from(if_: If) -> Self {
        Self::from((IrAstIf::from(if_), IrIfData::default()))
    }
}

impl From<(IrAstIf, IrIfData)> for IrIf {
    fn from((ast, data): (IrAstIf, IrIfData)) -> Self {
        Self(ast, data)
    }
}

impl From<If> for IrAstIf {
    fn from(
        If {
            if_tok,
            condition,
            left_brace,
            statements,
            right_brace,
        }: If,
    ) -> Self {
        Self {
            if_tok,
            condition: IrExpr::from(condition),
            left_brace,
            statements: statements.into_iter().map_into::<IrStatement>().collect(),
            right_brace,
        }
    }
}

//
// Else if
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrElseIf(#[whole_span] pub IrAstElseIf, pub IrElseIfData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrElseIfData;

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstElseIf {
    #[start_span]
    pub else_tok: Span,

    #[end_span]
    pub if_: IrIf,
}

impl_deref! {IrElseIf, IrAstElseIf}

impl From<ElseIf> for IrElseIf {
    fn from(else_if: ElseIf) -> Self {
        Self::from((IrAstElseIf::from(else_if), IrElseIfData::default()))
    }
}

impl From<(IrAstElseIf, IrElseIfData)> for IrElseIf {
    fn from((ast, data): (IrAstElseIf, IrElseIfData)) -> Self {
        Self(ast, data)
    }
}

impl From<ElseIf> for IrAstElseIf {
    fn from(ElseIf { else_tok, if_ }: ElseIf) -> Self {
        Self {
            else_tok,
            if_: IrIf::from(if_),
        }
    }
}

//
// Else
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrElse(#[whole_span] pub IrAstElse, pub IrElseData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrElseData;
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstElse {
    #[start_span]
    pub else_tok: Span,
    pub left_brace: Span,
    pub statements: Vec<IrStatement>,
    #[end_span]
    pub right_brace: Span,
}

impl_deref! {IrElse, IrAstElse}

impl From<Else> for IrElse {
    fn from(else_: Else) -> Self {
        Self::from((IrAstElse::from(else_), IrElseData::default()))
    }
}

impl From<(IrAstElse, IrElseData)> for IrElse {
    fn from((ast, data): (IrAstElse, IrElseData)) -> Self {
        Self(ast, data)
    }
}

impl From<Else> for IrAstElse {
    fn from(
        Else {
            else_tok,
            left_brace,
            statements,
            right_brace,
        }: Else,
    ) -> Self {
        Self {
            else_tok,
            left_brace,
            statements: statements.into_iter().map_into::<IrStatement>().collect(),
            right_brace,
        }
    }
}

//
// Conditional statement
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrConditionalStatement(
    #[whole_span] pub IrAstConditionalStatement,
    pub IrConditionalStatementData,
);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrConditionalStatementData(pub Ty);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IrAstConditionalStatement {
    pub initial_if: IrIf,
    pub else_ifs: Vec<IrElseIf>,
    pub else_: Option<IrElse>,
}

impl_deref! {IrConditionalStatement, IrAstConditionalStatement}

impl Loc for IrAstConditionalStatement {
    fn get_start(&self) -> usize {
        self.initial_if.get_start()
    }

    fn get_end(&self) -> usize {
        match &self.else_ {
            Some(else_) => else_.get_end(),
            None => match self.else_ifs.last() {
                Some(else_if) => else_if.get_end(),
                None => self.initial_if.get_end(),
            },
        }
    }
}

impl From<ConditionalStatement> for IrConditionalStatement {
    fn from(conditional_statement: ConditionalStatement) -> Self {
        Self::from((
            IrAstConditionalStatement::from(conditional_statement),
            IrConditionalStatementData::default(),
        ))
    }
}

impl From<(IrAstConditionalStatement, IrConditionalStatementData)> for IrConditionalStatement {
    fn from((ast, data): (IrAstConditionalStatement, IrConditionalStatementData)) -> Self {
        Self(ast, data)
    }
}

impl From<ConditionalStatement> for IrAstConditionalStatement {
    fn from(
        ConditionalStatement {
            initial_if,
            else_ifs,
            else_,
        }: ConditionalStatement,
    ) -> Self {
        Self {
            initial_if: IrIf::from(initial_if),
            else_ifs: else_ifs.into_iter().map_into::<IrElseIf>().collect(),
            else_: else_.map(IrElse::from),
        }
    }
}

//
// Foreach statement
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrForeachStatement(
    #[whole_span] pub IrAstForeachStatement,
    pub IrForeachStatementData,
);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrForeachStatementData;

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstForeachStatement {
    #[start_span]
    pub foreach_tok: Span,
    pub for_in_expr: IrForInExpr,
    pub left_brace: Span,
    pub statements: Vec<IrStatement>,
    #[end_span]
    pub right_brace: Span,
}

impl_deref! {IrForeachStatement, IrAstForeachStatement}

impl From<ForeachStatement> for IrForeachStatement {
    fn from(foreach_statement: ForeachStatement) -> Self {
        Self::from((
            IrAstForeachStatement::from(foreach_statement),
            IrForeachStatementData::default(),
        ))
    }
}

impl From<(IrAstForeachStatement, IrForeachStatementData)> for IrForeachStatement {
    fn from((ast, data): (IrAstForeachStatement, IrForeachStatementData)) -> Self {
        Self(ast, data)
    }
}

impl From<ForeachStatement> for IrAstForeachStatement {
    fn from(
        ForeachStatement {
            foreach_tok,
            for_in_expr,
            left_brace,
            statements,
            right_brace,
        }: ForeachStatement,
    ) -> Self {
        Self {
            foreach_tok,
            for_in_expr: IrForInExpr::from(for_in_expr),
            left_brace,
            statements: statements.into_iter().map_into::<IrStatement>().collect(),
            right_brace,
        }
    }
}

//
// For in expr
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrForInExpr(#[whole_span] pub IrAstForInExpr, pub IrForInExprData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrForInExprData(pub Ty);

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstForInExpr {
    #[start_span]
    pub name: Spanned<String>,
    pub in_tok: Span,
    #[end_span]
    pub expr: IrExpr,
}

impl_deref! {IrForInExpr, IrAstForInExpr}

impl From<ForInExpr> for IrForInExpr {
    fn from(for_in_expr: ForInExpr) -> Self {
        Self::from((
            IrAstForInExpr::from(for_in_expr),
            IrForInExprData::default(),
        ))
    }
}

impl From<(IrAstForInExpr, IrForInExprData)> for IrForInExpr {
    fn from((ast, data): (IrAstForInExpr, IrForInExprData)) -> Self {
        Self(ast, data)
    }
}

impl From<ForInExpr> for IrAstForInExpr {
    fn from(ForInExpr { name, in_tok, expr }: ForInExpr) -> Self {
        Self {
            name,
            in_tok,
            expr: IrExpr::from(expr),
        }
    }
}

//
// Control statement
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrControlStatement(
    #[whole_span] pub IrAstControlStatement,
    pub IrControlStatementData,
);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrControlStatementData(pub Option<Ty>);

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub enum IrAstControlStatement {
    Continue(#[start_span] Span, #[end_span] Span),
    Break(#[start_span] Span, #[end_span] Span),
    Return(#[start_span] Span, Option<Box<IrExpr>>, #[end_span] Span),
}

impl_deref! {IrControlStatement, IrAstControlStatement}

impl From<ControlStatement> for IrControlStatement {
    fn from(control_statement: ControlStatement) -> Self {
        Self::from((
            IrAstControlStatement::from(control_statement),
            IrControlStatementData::default(),
        ))
    }
}

impl From<(IrAstControlStatement, IrControlStatementData)> for IrControlStatement {
    fn from((ast, data): (IrAstControlStatement, IrControlStatementData)) -> Self {
        Self(ast, data)
    }
}

impl From<ControlStatement> for IrAstControlStatement {
    fn from(control_statement: ControlStatement) -> Self {
        match control_statement {
            ControlStatement::Continue(continue_, semicolon) => {
                Self::Continue(continue_, semicolon)
            }
            ControlStatement::Break(break_, semicolon) => Self::Break(break_, semicolon),
            ControlStatement::Return(return_, ret_expr, semicolon) => Self::Return(
                return_,
                ret_expr.map(|it| Box::new(IrExpr::from(it))),
                semicolon,
            ),
        }
    }
}

//
// Function body
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrFnBody(#[whole_span] pub IrAstFnBody, pub IrFnBodyData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrFnBodyData(pub Ty);
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstFnBody {
    #[start_span]
    pub left_brace: Span,
    pub statements: Vec<IrStatement>,
    pub tail_expr: Option<IrExpr>,
    #[end_span]
    pub right_brace: Span,
}

impl_deref! {IrFnBody, IrAstFnBody}

impl From<FnBody> for IrFnBody {
    fn from(fn_body: FnBody) -> Self {
        Self::from((IrAstFnBody::from(fn_body), IrFnBodyData::default()))
    }
}

impl From<(IrAstFnBody, IrFnBodyData)> for IrFnBody {
    fn from((ast, data): (IrAstFnBody, IrFnBodyData)) -> Self {
        Self(ast, data)
    }
}

impl From<FnBody> for IrAstFnBody {
    fn from(
        FnBody {
            left_brace,
            statements,
            tail_expr,
            right_brace,
        }: FnBody,
    ) -> Self {
        Self {
            left_brace,
            statements: statements.into_iter().map_into::<IrStatement>().collect(),
            tail_expr: tail_expr.map(IrExpr::from),
            right_brace,
        }
    }
}

//
// Type ref
//

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrTypeRef(#[whole_span] pub IrAstTypeRef, pub IrTypeRefData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrTypeRefData(pub Ty);

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub enum IrAstTypeRef {
    Named {
        #[whole_span]
        ty_name: Spanned<String>,
    },
    GenericNamed {
        #[start_span]
        ty_name: Spanned<String>,
        #[end_span]
        generics: IrTypeRefGenerics,
    },
    Fn {
        #[start_span]
        fn_span: Span,
        left_paren: Span,
        tys: Vec<IrTypeRef>,
        #[end_span]
        right_paren: Span,
    },
}

impl_deref! {IrTypeRef, IrAstTypeRef}

impl From<TypeRef> for IrTypeRef {
    fn from(type_ref: TypeRef) -> Self {
        Self::from((IrAstTypeRef::from(type_ref), IrTypeRefData::default()))
    }
}

impl From<(IrAstTypeRef, IrTypeRefData)> for IrTypeRef {
    fn from((ast, data): (IrAstTypeRef, IrTypeRefData)) -> Self {
        Self(ast, data)
    }
}

impl From<TypeRef> for IrAstTypeRef {
    fn from(type_ref: TypeRef) -> Self {
        match type_ref {
            TypeRef::Named { ty_name } => Self::Named { ty_name },
            TypeRef::GenericNamed { ty_name, generics } => Self::GenericNamed {
                ty_name,
                generics: IrTypeRefGenerics::from(generics),
            },
            TypeRef::Fn {
                fn_span,
                left_paren,
                tys,
                right_paren,
            } => Self::Fn {
                fn_span,
                left_paren,
                tys: tys.into_iter().map_into::<IrTypeRef>().collect(),
                right_paren,
            },
        }
    }
}

//
// Type ref generics
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrTypeRefGenerics(#[whole_span] IrAstTypeRefGenerics, IrTypeRefGenericsData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrTypeRefGenericsData(Vec<Ty>);

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstTypeRefGenerics {
    #[start_span]
    pub less_than: Span,
    pub tys: Vec<IrTypeRef>,
    #[end_span]
    pub greater_than: Span,
}

impl_deref! {IrTypeRefGenerics, IrAstTypeRefGenerics}

impl From<TypeRefGenerics> for IrTypeRefGenerics {
    fn from(type_ref_generics: TypeRefGenerics) -> Self {
        Self::from((
            IrAstTypeRefGenerics::from(type_ref_generics),
            IrTypeRefGenericsData::default(),
        ))
    }
}

impl From<(IrAstTypeRefGenerics, IrTypeRefGenericsData)> for IrTypeRefGenerics {
    fn from((ast, data): (IrAstTypeRefGenerics, IrTypeRefGenericsData)) -> Self {
        Self(ast, data)
    }
}

impl From<TypeRefGenerics> for IrAstTypeRefGenerics {
    fn from(
        TypeRefGenerics {
            less_than,
            tys,
            greater_than,
        }: TypeRefGenerics,
    ) -> Self {
        Self {
            less_than,
            tys: tys.into_iter().map_into::<IrTypeRef>().collect(),
            greater_than,
        }
    }
}

//
// Positional param
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrPositionalParam(
    #[whole_span] pub IrAstPositionalParam,
    pub IrPositionalParamData,
);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrPositionalParamData;

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstPositionalParam {
    #[start_span]
    pub name: Spanned<String>,
    pub colon: Span,
    #[end_span]
    pub ty: IrTypeRef,
}

impl_deref! {IrPositionalParam, IrAstPositionalParam}

impl From<PositionalParam> for IrPositionalParam {
    fn from(param: PositionalParam) -> Self {
        Self::from((
            IrAstPositionalParam::from(param),
            IrPositionalParamData::default(),
        ))
    }
}

impl From<(IrAstPositionalParam, IrPositionalParamData)> for IrPositionalParam {
    fn from((ast, data): (IrAstPositionalParam, IrPositionalParamData)) -> Self {
        Self(ast, data)
    }
}

impl From<PositionalParam> for IrAstPositionalParam {
    fn from(PositionalParam { name, colon, ty }: PositionalParam) -> Self {
        Self {
            name,
            colon,
            ty: IrTypeRef::from(ty),
        }
    }
}

//
// Default param
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrDefaultParam(#[whole_span] pub IrAstDefaultParam, pub IrDefaultParamData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrDefaultParamData;

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstDefaultParam {
    #[start_span]
    pub name: Spanned<String>,
    pub colon: Span,
    pub ty: IrTypeRef,
    pub eq: Span,
    #[end_span]
    pub value: IrExpr,
}

impl_deref! {IrDefaultParam, IrAstDefaultParam}

impl From<DefaultParam> for IrDefaultParam {
    fn from(param: DefaultParam) -> Self {
        Self::from((
            IrAstDefaultParam::from(param),
            IrDefaultParamData::default(),
        ))
    }
}

impl From<(IrAstDefaultParam, IrDefaultParamData)> for IrDefaultParam {
    fn from((ast, data): (IrAstDefaultParam, IrDefaultParamData)) -> Self {
        Self(ast, data)
    }
}

impl From<DefaultParam> for IrAstDefaultParam {
    fn from(
        DefaultParam {
            name,
            colon,
            ty,
            eq,
            value,
        }: DefaultParam,
    ) -> Self {
        Self {
            name,
            colon,
            ty: IrTypeRef::from(ty),
            eq,
            value: IrExpr::from(value),
        }
    }
}

//
// FnDecl
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrFnDecl(#[whole_span] pub IrAstFnDecl, pub IrFnDeclData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrFnDeclData;

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstFnDecl {
    #[start_span]
    pub fn_span: Span,
    pub name: Spanned<String>,
    pub left_paren: Span,
    pub positional_params: Vec<IrPositionalParam>,
    pub default_params: Vec<IrDefaultParam>,
    pub right_paren: Span,
    #[end_span]
    pub body: IrFnBody,
}

impl_deref! {IrFnDecl, IrAstFnDecl}

impl From<FnDecl> for IrFnDecl {
    fn from(fn_decl: FnDecl) -> Self {
        Self::from((IrAstFnDecl::from(fn_decl), IrFnDeclData::default()))
    }
}

impl From<(IrAstFnDecl, IrFnDeclData)> for IrFnDecl {
    fn from((ast, data): (IrAstFnDecl, IrFnDeclData)) -> Self {
        Self(ast, data)
    }
}
impl From<FnDecl> for IrAstFnDecl {
    fn from(
        FnDecl {
            fn_span,
            name,
            left_paren,
            positional_params,
            default_params,
            right_paren,
            body,
        }: FnDecl,
    ) -> Self {
        Self {
            fn_span,
            name,
            left_paren,
            positional_params: positional_params
                .into_iter()
                .map_into::<IrPositionalParam>()
                .collect(),
            default_params: default_params
                .into_iter()
                .map_into::<IrDefaultParam>()
                .collect(),
            right_paren,
            body: IrFnBody::from(body),
        }
    }
}

//
// Lang item
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrLangItem(#[whole_span] pub IrAstLangItem, pub IrLangItemData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrLangItemData;
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub enum IrAstLangItem {
    FnDecl(#[whole_span] IrFnDecl),
    Statement(#[whole_span] Box<IrStatement>),
}

impl_deref! {IrLangItem, IrAstLangItem}

impl From<LangItem> for IrLangItem {
    fn from(lang_item: LangItem) -> Self {
        Self::from((IrAstLangItem::from(lang_item), IrLangItemData::default()))
    }
}

impl From<(IrAstLangItem, IrLangItemData)> for IrLangItem {
    fn from((ast, data): (IrAstLangItem, IrLangItemData)) -> Self {
        Self(ast, data)
    }
}

impl From<LangItem> for IrAstLangItem {
    fn from(lang_item: LangItem) -> Self {
        match lang_item {
            LangItem::FnDecl(fn_decl) => Self::FnDecl(IrFnDecl::from(fn_decl)),
            LangItem::Statement(statement) => {
                Self::Statement(Box::new(IrStatement::from(statement)))
            }
        }
    }
}

//
// Statement
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrStatement(#[whole_span] pub IrAstStatement, pub IrStatementData);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrStatementData;

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub enum IrAstStatement {
    ExecExpr(#[whole_span] IrExprStatement),
    Declaration(#[whole_span] IrDeclaration),
    Assignment(#[whole_span] Box<IrAssignment>),
    Conditional(#[whole_span] IrConditionalStatement),
    Control(#[whole_span] IrControlStatement),
    Foreach(#[whole_span] IrForeachStatement),
}

impl_deref! {IrStatement, IrAstStatement}

impl From<Statement> for IrStatement {
    fn from(statement: Statement) -> Self {
        Self::from((IrAstStatement::from(statement), IrStatementData::default()))
    }
}

impl From<(IrAstStatement, IrStatementData)> for IrStatement {
    fn from((ast, data): (IrAstStatement, IrStatementData)) -> Self {
        Self(ast, data)
    }
}

impl From<Statement> for IrAstStatement {
    fn from(statement: Statement) -> Self {
        match statement {
            Statement::ExecExpr(exec) => Self::ExecExpr(IrExprStatement::from(exec)),
            Statement::Declaration(decl) => Self::Declaration(IrDeclaration::from(decl)),
            Statement::Assignment(assignment) => {
                Self::Assignment(Box::new(IrAssignment::from(assignment)))
            }
            Statement::Conditional(conditional) => {
                Self::Conditional(IrConditionalStatement::from(conditional))
            }
            Statement::Control(control) => Self::Control(IrControlStatement::from(control)),
            Statement::Foreach(foreach) => Self::Foreach(IrForeachStatement::from(foreach)),
        }
    }
}

//
// Build definition
//
#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrBuildDefinition(
    #[whole_span] pub IrAstBuildDefinition,
    pub IrBuildDefinitionData,
);
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IrBuildDefinitionData;

#[derive(Loc, Clone, Debug, Eq, PartialEq)]
pub struct IrAstBuildDefinition {
    #[whole_span]
    pub items: Vec<IrLangItem>,
}

impl_deref! {IrBuildDefinition, IrAstBuildDefinition}

impl From<BuildDefinition> for IrBuildDefinition {
    fn from(build_definition: BuildDefinition) -> Self {
        Self::from((
            IrAstBuildDefinition::from(build_definition),
            IrBuildDefinitionData::default(),
        ))
    }
}

impl From<(IrAstBuildDefinition, IrBuildDefinitionData)> for IrBuildDefinition {
    fn from((ast, data): (IrAstBuildDefinition, IrBuildDefinitionData)) -> Self {
        Self(ast, data)
    }
}

impl From<BuildDefinition> for IrAstBuildDefinition {
    fn from(BuildDefinition { items }: BuildDefinition) -> Self {
        Self {
            items: items.into_iter().map_into::<IrLangItem>().collect(),
        }
    }
}
