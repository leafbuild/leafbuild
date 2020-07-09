mod lexer;
/// the parser
mod sbuild;
pub mod ast;

pub fn parse(source: &str) {
    let program = sbuild::ExprParser::new().parse(lexer::Lexer::new(source)).unwrap();
}