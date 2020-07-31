pub mod ast;
/// the parser
mod leafparser;
pub(crate) mod lexer;

use crate::grammar::lexer::Tok;
use lalrpop_util::ParseError;
pub use lexer::TokLoc;

pub fn parse(source: &str) -> Result<ast::AstProgram, ParseError<usize, Tok, lexer::LexicalError>> {
    let statements = leafparser::ProgramParser::new().parse(lexer::Lexer::new(source))?;
    Ok(ast::AstProgram::new(statements))
}
