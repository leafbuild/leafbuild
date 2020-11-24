pub(super) mod fun;
pub(super) mod values;

use crate::grammar::ast::Statement;
use crate::interpreter::env::FileFrame;

pub(super) fn run_statement(frame: &mut FileFrame, statement: &Statement) {
    trace!("Executing statement at {:#?}", statement);
    match statement {
        Statement::FuncCall(call) => {
            // _frame.with_context(|context| call.get_rng())
            println!("Func call {}", call.get_name());

            // call.lookup_in_context(frame);
        }
        Statement::MethodCall(_)
        | Statement::Declaration(_)
        | Statement::Assignment(_)
        | Statement::Conditional(_)
        | Statement::Control(_)
        | Statement::Repetitive(_) => {}
    }
}
