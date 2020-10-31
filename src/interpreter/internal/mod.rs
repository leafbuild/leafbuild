use crate::grammar::ast::{AstLoc, AstStatement};
use crate::interpreter::env::EnvFrame;

pub(crate) fn run_statement(_frame: &mut EnvFrame, statement: &AstStatement) {
    trace!("Executing statement at {:#?}", statement.get_rng());
    match statement {
        AstStatement::FuncCall(call) => println!("Func call {}", call.get_name()),
        AstStatement::MethodCall(_) => {}
        AstStatement::Declaration(_) => {}
        AstStatement::Assignment(_) => {}
        AstStatement::Conditional(_) => {}
        AstStatement::ControlStatement(_) => {}
        AstStatement::Repetitive(_) => {}
    }
}
