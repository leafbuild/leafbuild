// Those are only *parsing* tests, so as long as they make sense
// *syntactically*, they should parse.
// Even if semantically they are void of meaning.

mod parse_output {
    use crate::parser::parse;
    use crate::syn_tree::{self, AstNode, CastableFromSyntaxNode, Root, SyntaxElement};
    use crate::LeafbuildLanguage;
    macro_rules! parse_test {
        ($name:ident, $s:literal) => {
            let mut s = String::new();
            let p = parse($s);
            assert_eq!(p.errors, vec![]);
            let node = rowan::SyntaxNode::<LeafbuildLanguage>::new_root(p.green_node);
            let node: Root = Root::cast(node).unwrap();
            syn_tree::test_dbg(0, SyntaxElement::Node(node.syntax().clone()), &mut s);
            insta::assert_snapshot!(stringify!($name), s.as_str(), $s);
        };
    }

    macro_rules! parse_test_full {
        ($name:ident, $s:literal) => {
            #[test]
            fn $name() {
                parse_test!($name, $s);
            }
        };
    }

    parse_test_full!(declaration, "let a = b\n");
    parse_test_full!(assignment, "a = b\n");
    parse_test_full!(unit_to_unit_assignment, "() = ()\n");
    parse_test_full!(line_trailling_space, "() = () \n");
    parse_test_full!(if_condition, "if (()) {} else {}");
    parse_test_full!(if_else_if_condition, "if () {} else if () {} else {}");
    parse_test_full!(
        if_else_if_else_condition_stretched,
        r#"
        if ()
    
        {
    
    
    
        }
    
        else if () {
    
    
        }
    
        else {
    
    
        }
    "#
    );
    parse_test_full!(if_condition_in_expr, "() = if () {} else {} \n");
    parse_test_full!(
        if_condition_in_expr_stretched,
        r#"
        () = if ()
        {
    
        }
    
        else
        {
    
    
        }
    "#
    );
    parse_test_full!(foreach_basic, "foreach () in () {}");
    parse_test_full!(freestanding_expr, "1\n");
    parse_test_full!(var_assignment_with_proper_expr_as_value, "let x = 1\n");
    parse_test_full!(precedence_parsing, "x = 1 + 2 * 3 % - 4 ( 5 )\n");
}
