pub mod ast;
/// the parser
#[allow(clippy::all)]
mod leafparser;
pub(crate) mod lexer;

use crate::grammar::lexer::LexicalError;
use lalrpop_util::ParseError;
pub use lexer::Span;

pub fn parse(
    source: &str,
) -> Result<ast::AstBuildDefinition, ParseError<usize, lexer::Token, LexicalError>> {
    leafparser::ProgramParser::new()
        .parse(source, lexer::Lexer::new(source))
        .map(ast::AstBuildDefinition::new)
}
