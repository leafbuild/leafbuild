use itertools::Itertools;

fn main() {
    use leafbuild_syntax::parser::*;

    let parsed = parse("let a = ( (), (), \n");
    println!("{:#?}", &parsed);
    println!(
        "{}",
        parsed
            .green_node
            .children()
            .map(|it| format!("{:#?}", it.as_node().unwrap()))
            .join(", ")
    )
}
