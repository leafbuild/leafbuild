use std::error::Error;

pub mod ast;
/// the parser
mod cmbs;
mod lexer;

pub fn parse(source: &str) -> Result<ast::AstProgram, Box<dyn Error>> {
    let statements = cmbs::ProgramParser::new().parse(lexer::Lexer::new(source))?;
    Ok(ast::AstProgram::new(statements))
}
