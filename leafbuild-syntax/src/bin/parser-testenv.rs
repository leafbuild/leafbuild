use rowan::NodeOrToken;

fn main() {
    use leafbuild_syntax::parser::*;

    let parsed = parse(
        r#"let x = y + z * c + d + {
            f()
            a
        }
    "#,
    );
    println!("{:#?}", &parsed);
    let node = rowan::SyntaxNode::new_root(parsed.green_node);
    leafbuild_syntax::syn_tree::print(0, NodeOrToken::Node(node));
}
