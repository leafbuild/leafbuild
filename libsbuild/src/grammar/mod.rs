use itertools::Itertools;

mod lexer;
/// the parser

mod sbuild;
pub mod ast;

pub fn parse(source: &str) {
    let program = sbuild::ProgramParser::new().parse(lexer::Lexer::new(source)).unwrap();
}