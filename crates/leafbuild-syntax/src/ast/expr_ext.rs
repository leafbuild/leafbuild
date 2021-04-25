#![allow(missing_docs)]
use crate::T;

use super::SyntaxToken;
use crate::ast::{self, support, AstNode};
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinOp {
    /// The `==` operator for equality testing
    EqualityTest,
    /// The `!=` operator for equality testing
    NegatedEqualityTest,
    /// The `<=` operator for lesser-equal testing
    LesserEqualTest,
    /// The `>=` operator for greater-equal testing
    GreaterEqualTest,
    /// The `<` operator for comparison
    LesserTest,
    /// The `>` operator for comparison
    GreaterTest,
    /// The `+` operator for addition
    Addition,
    /// The `*` operator for multiplication
    Multiplication,
    /// The `-` operator for subtraction
    Subtraction,
    /// The `/` operator for division
    Division,
    /// The `%` operator for remainder after division
    Remainder,
    /// The `=` operator for assignment
    Assignment,
    /// The `+=` operator for assignment after addition
    AddAssign,
    /// The `/=` operator for assignment after division
    DivAssign,
    /// The `*=` operator for assignment after multiplication
    MulAssign,
    /// The `%=` operator for assignment after remainders
    RemAssign,
    /// The `-=` operator for assignment after subtraction
    SubAssign,
}

impl BinOp {
    pub fn is_assignment(self) -> bool {
        matches!(
            self,
            BinOp::Assignment
                | BinOp::AddAssign
                | BinOp::DivAssign
                | BinOp::MulAssign
                | BinOp::RemAssign
                | BinOp::SubAssign
        )
    }
}

impl ast::BinExpr {
    pub fn op_details(&self) -> Option<(SyntaxToken, BinOp)> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find_map(|c| {
                let bin_op = match c.kind() {
                    T![==] => BinOp::EqualityTest,
                    T![!=] => BinOp::NegatedEqualityTest,
                    T![<=] => BinOp::LesserEqualTest,
                    T![>=] => BinOp::GreaterEqualTest,
                    T![<] => BinOp::LesserTest,
                    T![>] => BinOp::GreaterTest,
                    T![+] => BinOp::Addition,
                    T![*] => BinOp::Multiplication,
                    T![-] => BinOp::Subtraction,
                    T![/] => BinOp::Division,
                    T![%] => BinOp::Remainder,
                    T![=] => BinOp::Assignment,
                    T![+=] => BinOp::AddAssign,
                    T![/=] => BinOp::DivAssign,
                    T![*=] => BinOp::MulAssign,
                    T![%=] => BinOp::RemAssign,
                    T![-=] => BinOp::SubAssign,
                    _ => return None,
                };
                Some((c, bin_op))
            })
    }

    pub fn op_kind(&self) -> Option<BinOp> {
        self.op_details().map(|t| t.1)
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.op_details().map(|t| t.0)
    }

    pub fn lhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).next()
    }

    pub fn rhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).nth(1)
    }

    pub fn sub_exprs(&self) -> (Option<ast::Expr>, Option<ast::Expr>) {
        let mut children = support::children(self.syntax());
        let first = children.next();
        let second = children.next();
        (first, second)
    }
}
