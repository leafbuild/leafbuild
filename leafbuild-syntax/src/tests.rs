fn parses_without_errors(s: &str) {
    assert_eq!(crate::parser::parse(s).errors, vec![]);
}

// Those are only *parsing* tests, so as long as they make sense
// *syntactically*, they should parse.
// Even if semantically they are void of meaning.
#[test]
fn unit_to_unit_assignment() {
    parses_without_errors("() = ()\n")
}

#[test]
fn line_trailing_space() {
    parses_without_errors("() = () \n")
}

#[test]
fn if_condition() {
    parses_without_errors("if (()) {} else {}")
}

#[test]
fn if_else_if_condition() {
    parses_without_errors("if () {} else if () {} else {}")
}

#[test]
fn if_else_if_else_condition_stretched() {
    parses_without_errors(
        r#"
    if ()

    {



    }

    else if () {


    }

    else {


    }
    "#,
    )
}

#[test]
fn if_condition_in_expr() {
    parses_without_errors("() = if () {} else {} \n")
}

#[test]
fn if_condition_in_expr_stretched() {
    parses_without_errors(
        r#"
    () = if ()
    {

    }

    else
    {


    }
    "#,
    )
}

#[test]
fn declaration() {
    parses_without_errors("let x = ()\n")
}
