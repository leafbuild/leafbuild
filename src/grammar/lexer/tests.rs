pub use super::*;
pub use unindent::Unindent;

macro_rules! lexer_test {
    ($input:expr, [$({$start:expr, $token:expr, $end:expr}),* $(,)?]) => {
        let lexer = Lexer::new($input);
        let expected = vec![
            $(Ok((
                    $start,
                    $token,
                    $end,
                )),
            )*
        ];
        itertools::assert_equal(lexer, expected.into_iter());
    };
}

mod single_token {
    use super::*;

    macro_rules! single_token_test {
        ($input:expr, rf, $ty:expr) => {
            let input = $input;
            lexer_test!(&input, [{0, Token{token: $ty, data: &input}, input.len()}]);
        };
        ($input:literal, $ty:expr) => {
            let input = $input;
            lexer_test!(input, [{0, Token{token: $ty, data: input}, input.len()}]);
        };
    }

    macro_rules! single_token_test_fn {
        ($name:ident, $input:expr, rf, $ty:expr) => {
            #[test]
            fn $name() {
                single_token_test!($input, rf, $ty);
            }
        };
        ($name:ident, $input:literal, $ty:expr) => {
            #[test]
            fn $name() {
                single_token_test!($input, $ty);
            }
        };
    }

    single_token_test_fn! {simple_string, r#"'a'"#, Tk::String}
    single_token_test_fn! {simple_string_with_escape, r#"'a\'b'"#, Tk::String}
    single_token_test_fn! {multiline_string, "
                                             '''a
                                             b
                                             c'''".unindent(), rf, Tk::MultilineString}
    single_token_test_fn! {multiline_string_with_apostrophes,
        "
        '''
        a
        '
        b
        ''
        c
        '
        '''"
        .unindent(),
        rf,
        Tk::MultilineString
    }

    single_token_test_fn! {line_comment, "//a b c", Tk::SingleLineComment}
    // single_token_test_fn! {block_comment, "/*a b c*/", Tk::BlockComment}
    // single_token_test_fn! {block_comment_with_asterisk, "/*a b c ****** g h i */", Tk::BlockComment }
}
