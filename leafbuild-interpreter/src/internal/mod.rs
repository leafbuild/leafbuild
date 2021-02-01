pub mod eval;
pub(super) mod fun;
pub(super) mod values;

use crate::env::FileFrame;
use crate::internal::eval::Eval;
use leafbuild_ast::ast::{BuildDefinition, Loc, Statement};

#[allow(clippy::needless_pass_by_value)]
pub(super) fn run_build_def<'frame>(
    frame: &'frame mut FileFrame<'frame>,
    build_def: BuildDefinition,
) {
    // build_def.items.iter().for_each(|it| match it {
    //     LangItem::FnDecl(fn_decl) => frame.index(fn_decl),
    //     LangItem::Statement(_) => (),
    // });
    // build_def
    //     .items
    //     .iter()
    //     .for_each(|statement| run_statement(frame, statement))
}

fn run_statement<'frame>(frame: &'frame mut FileFrame<'_>, statement: &Statement) {
    trace!(
        "Executing statement at {:?}\nStatement = {:#?}",
        statement.get_rng(),
        statement
    );
    match statement {
        Statement::ExecExpr(ref exp) => {
            exp.eval_in_context(frame);
        }
        Statement::Declaration(decl) => {}
        Statement::Assignment(_)
        | Statement::Conditional(_)
        | Statement::Control(_)
        | Statement::Repetitive(_) => {}
    }
}
