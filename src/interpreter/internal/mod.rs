pub(super) mod properties;
pub(super) mod values;

use crate::grammar::ast::{Loc, Statement};
use crate::interpreter::env::FileFrame;

pub(super) fn run_statement(_frame: &mut FileFrame, statement: &Statement) {
    trace!("Executing statement at {:#?}", statement.get_rng());
    match statement {
        Statement::FuncCall(call) => {
            // _frame.with_context(|context| call.get_rng())
            // println!("Func call {}", call.get_name())
        }
        Statement::MethodCall(_)
        | Statement::Declaration(_)
        | Statement::Assignment(_)
        | Statement::Conditional(_)
        | Statement::Control(_)
        | Statement::Repetitive(_) => {}
    }
}
