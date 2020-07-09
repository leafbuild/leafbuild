use itertools::Itertools;

mod lexer;
/// the parser

mod sbuild;
pub mod ast;

pub fn parse(source: &str) {
    let program = sbuild::ProgramParser::new().parse(lexer::Lexer::new(source)).unwrap();
    program.iter().for_each(|x| {
        println!("calling function '{}' with positional args '{}' and named args '{}'",
                 x.get_name(),
                 "",
                 x.get_args().get_named_args().iter().map(|arg| { arg.get_name() }).join(", ")
        );
    });
}