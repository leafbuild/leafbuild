use itertools::Itertools;

fn main() {
    use leafbuild_syntax::parser::*;

    let parsed = parse(
        r#"

    (  )=if (){}

    if ((), ()) {

    }
    else if (          ()       )

    {

    }
    else
    {

    }
    "#,
    );
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
