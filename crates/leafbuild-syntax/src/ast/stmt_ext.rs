use crate::SyntaxKind::{self, *};

use super::{
    AssignmentStmt, AstNode, ConditionalStmt, ControlStmt, DeclarationStmt, ExprEvalStmt,
    ForeachStmt, Stmt, SyntaxNode,
};

impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EXPR_EVAL_STMT | DECLARATION_STMT | ASSIGNMENT_STMT | FOREACH_STMT
            | CONDITIONAL_STMT => true,
            kind if ControlStmt::can_cast(kind) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            EXPR_EVAL_STMT => Stmt::ExprEvalStmt(ExprEvalStmt { syntax }),
            DECLARATION_STMT => Stmt::DeclarationStmt(DeclarationStmt { syntax }),
            ASSIGNMENT_STMT => Stmt::AssignmentStmt(AssignmentStmt { syntax }),
            FOREACH_STMT => Stmt::ForeachStmt(ForeachStmt { syntax }),
            CONDITIONAL_STMT => Stmt::ConditionalStmt(ConditionalStmt { syntax }),
            kind if ControlStmt::can_cast(kind) => {
                Stmt::ControlStmt(ControlStmt::cast(syntax).unwrap())
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::ExprEvalStmt(it) => &it.syntax,
            Stmt::DeclarationStmt(it) => &it.syntax,
            Stmt::AssignmentStmt(it) => &it.syntax,
            Stmt::ForeachStmt(it) => &it.syntax,
            Stmt::ConditionalStmt(it) => &it.syntax,
            Stmt::ControlStmt(it) => it.syntax(),
        }
    }
}
