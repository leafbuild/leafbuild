use leafbuild_syntax::syn_tree::{AstNode, CastableFromSyntaxNode, Root, SyntaxElement};
use leafbuild_syntax::LeafbuildLanguage;
use rowan::NodeOrToken;

fn main() {
    use leafbuild_syntax::parser::*;

    let parsed = parse(
        r#"let x = { 1 + 0 }
        x + 1
    "#,
    );
    println!("{:#?}", &parsed);
    let node = rowan::SyntaxNode::<LeafbuildLanguage>::new_root(parsed.green_node);
    let node: Root = Root::cast(node).unwrap();
    leafbuild_syntax::syn_tree::print(0, SyntaxElement::Node(node.syntax().clone()));
    node.lang_items().for_each(|lang_item| {
        leafbuild_syntax::syn_tree::print(
            0,
            SyntaxElement::Node(
                lang_item
                    .as_statement()
                    .unwrap()
                    .as_declaration()
                    .unwrap()
                    .syntax()
                    .clone(),
            ),
        );
    })
}
