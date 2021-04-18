// use crate::syn_tree::AstNode;

#[test]
fn visit_root() {
    use crate::syn_tree::visitor::Visitor;
    use crate::syn_tree::Root;
    use crate::LeafbuildLanguage;
    use indoc::indoc;

    struct V(bool);

    impl Default for V {
        fn default() -> Self {
            Self(false)
        }
    }

    impl Visitor for V {
        type Output = ();
        fn undefined(&mut self, _name: &'static str) {}

        fn visit_root(&mut self, _root: Root) -> Self::Output {
            self.0 = true;
        }
    }
    let (node, errors) = crate::parser::parse(indoc!(
        r#"
        let a = 1
        let b = 2
        a + b
        "#
    ));
    assert!(errors.is_empty());
    let node = rowan::SyntaxNode::<LeafbuildLanguage>::new_root(node);

    let node = node;

    let mut vis = V::default();
    assert_eq!(vis.0, false);
    vis.visit(node);
    assert_eq!(vis.0, true);
}

#[test]
fn visit_root_undefined_but_undefined_overriden() {
    use crate::syn_tree::visitor::Visitor;
    use crate::LeafbuildLanguage;
    use indoc::indoc;

    struct V;

    impl Visitor for V {
        type Output = ();
        fn undefined(&mut self, _name: &'static str) {}
    }
    let (node, errors) = crate::parser::parse(indoc!(
        r#"
        let a = 1
        let b = 2
        a + b
        "#
    ));
    assert!(errors.is_empty());
    let node = rowan::SyntaxNode::<LeafbuildLanguage>::new_root(node);

    let node = node;

    let mut vis = V;
    vis.visit(node);
}

#[test]
#[should_panic = "Visitor doesn't implement visit_root"]
fn visit_root_undefined_but_undefined_not_overriden() {
    use crate::syn_tree::visitor::Visitor;
    use crate::LeafbuildLanguage;
    use indoc::indoc;

    struct V;

    impl Visitor for V {
        type Output = ();
    }
    let (node, errors) = crate::parser::parse(indoc!(
        r#"
        let a = 1
        let b = 2
        a + b
        "#
    ));
    assert!(errors.is_empty());
    let node = rowan::SyntaxNode::<LeafbuildLanguage>::new_root(node);

    let node = node;

    let mut vis = V;
    vis.visit(node);
}
