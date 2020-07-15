use itertools::Itertools;
use std::error::Error;

mod lexer;
/// the parser

mod sbuild;
pub mod ast;

pub fn parse(source: &str) -> Result<ast::AstProgram, Box<dyn Error>> {
    let statements = sbuild::ProgramParser::new().parse(lexer::Lexer::new(source))?;
    Ok(ast::AstProgram::new(statements))
}