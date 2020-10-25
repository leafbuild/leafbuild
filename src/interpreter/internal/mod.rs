use crate::grammar::ast::AstStatement;
use crate::interpreter::env::EnvFrame;

pub(crate) fn run_statement(frame: &mut EnvFrame, statement: &AstStatement) {
    debug!("Executing statement {:#?}", statement);
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
