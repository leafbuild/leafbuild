pub mod eval;
pub(super) mod fun;
pub(super) mod values;

use crate::env::FileFrame;
use leafbuild_ast::ast::{Loc, Statement};

pub(super) fn run_statement(frame: &mut FileFrame, statement: &Statement) {
    trace!(
        "Executing statement at {:?}\nStatement = {:#?}",
        statement.get_rng(),
        statement
    );
    match statement {
        Statement::ExecExpr(ref exp) => {
            // exp.eval_in_context(frame).
        }
        Statement::Declaration(_)
        | Statement::Assignment(_)
        | Statement::Conditional(_)
        | Statement::Control(_)
        | Statement::Repetitive(_) => {}
    }
}
