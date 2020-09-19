pub mod ast;
/// the parser
mod leafparser;
pub(crate) mod lexer {
    #[derive(Copy, Clone, std::fmt::Debug)]
    pub struct TokLoc {
        begin: usize,
        end: usize,
    }

    impl TokLoc {
        #[inline]
        pub(crate) fn new(begin: usize, end: usize) -> TokLoc {
            TokLoc { begin, end }
        }
        #[inline]
        pub(crate) fn as_rng(&self) -> std::ops::Range<usize> {
            self.begin..self.end
        }

        #[inline]
        pub(crate) fn get_begin(&self) -> usize {
            self.begin
        }

        #[inline]
        pub(crate) fn get_end(&self) -> usize {
            self.end
        }
    }
}

use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
pub use lexer::TokLoc;

pub fn parse<'input>(
    source: &'input str,
) -> Result<ast::AstProgram, ParseError<usize, Token<'input>, &'static str>> {
    let statements = leafparser::ProgramParser::new().parse(source)?;
    Ok(ast::AstProgram::new(statements))
}
