//! # The parser code
//! Also see [the syntax reference](https://leafbuild.github.io/leafbuild/dev/syntax_ref.html).

use std::fmt;
use std::ops::Range;

use rowan::{TextRange, TextSize};
use text_token_source::LexerWrap;

use leafbuild_stdx::CopyTo;

use crate::lexer::Lexer;
use crate::SyntaxKind::{self, *};
use crate::T;

use self::text_tree_sink::TextTreeSink;
use self::token_source::{FindProperty, ForwardToken, Token};
use self::tree_sink::TreeSink;
use self::{
    error::ParseError,
    event::Event,
    marker::{CompletedMarker, Marker},
    token_source::TokenSource,
};

pub mod error;
mod event;
mod marker;
pub mod text_token_source;
pub mod text_tree_sink;
pub mod token_source;
pub mod tree_sink;

trait IsTrivia: Copy {
    fn is_trivia(self) -> bool;
}

impl IsTrivia for SyntaxKind {
    fn is_trivia(self) -> bool {
        self == Self::WHITESPACE || self == Self::COMMENT || self == Self::BLOCK_COMMENT
    }
}

/// A span in the source code
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Span {
    /// The underlying text range
    pub text_range: TextRange,
}

impl From<Range<TextSize>> for Span {
    fn from(range: Range<TextSize>) -> Self {
        Self {
            text_range: TextRange::new(range.start, range.end),
        }
    }
}

impl From<Range<u32>> for Span {
    fn from(range: Range<u32>) -> Self {
        Self {
            text_range: TextRange::new(range.start.into(), range.end.into()),
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.text_range.fmt(f)
    }
}

pub(crate) struct Parser<'ts> {
    /// tokens
    source: &'ts mut dyn TokenSource,
    events: Vec<Event>,
}

#[allow(clippy::inline_always)]
impl<'ts> Parser<'ts> {
    fn parse(&mut self) {
        let marker = self.start();
        loop {
            if self.at(EOF) {
                break;
            }
            parse_lang_item(self);
        }

        marker.complete(self, BUILD_DEFINITION);
    }

    #[inline(always)]
    fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    #[inline(always)]
    fn at_any<const N: usize>(&self, kinds: [SyntaxKind; N]) -> bool {
        let current = self.current();
        std::array::IntoIter::new(kinds).any(|it| it == current)
    }

    #[inline(always)]
    fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    #[inline(always)]
    fn bump_any(&mut self) {
        let kind = self.current();
        if kind == EOF {
            return;
        }

        self.do_bump(kind, 1);
    }

    fn do_bump(&mut self, kind: SyntaxKind, n_raw_tokens: u8) {
        for _ in 0..n_raw_tokens {
            self.source.bump();
        }
        self.push_event(Event::Token { kind, n_raw_tokens })
    }

    fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(format!("expected {:?}, got {:?}", kind, self.current()));
        false
    }

    pub(crate) fn error<E: Into<ParseError>>(&mut self, msg: E) {
        let msg = msg.into();
        self.push_event(Event::Error { msg })
    }

    #[inline(always)]
    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.do_bump(kind, 1);
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    #[inline(always)]
    fn nth(&self, n: usize) -> SyntaxKind {
        self.source.lookahead(n).syntax_kind
    }

    #[inline(always)]
    fn require_newline(&mut self) {
        self.expect(T![newline]);
    }

    #[inline(always)]
    fn has_newline(&self) -> bool {
        self.at(T![newline])
    }

    #[inline(always)]
    fn bump_to(&mut self, forward_token: ForwardToken) {
        let bumps = self.source.bump_to(forward_token);
        for _ in 0..bumps {
            self.bump_any();
        }
    }

    #[inline(always)]
    fn next_not_newline(&self) -> ForwardToken {
        self.source.find(FindProperty::KindIsNot(T![newline]))
    }

    #[inline(always)]
    fn bump_to_if_next_non_newline_is(&mut self, kind: SyntaxKind) -> bool {
        let mut tk = ForwardToken::default();
        let k = self.next_not_newline().copy_to(&mut tk).kind;
        if k == kind {
            self.bump_to(tk);
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn bump_to_if_next_non_newline_is_any<const KINDS_SIZE: usize>(
        &mut self,
        kinds: [SyntaxKind; KINDS_SIZE],
    ) -> bool {
        let mut tk = ForwardToken::default();
        let k = self.next_not_newline().copy_to(&mut tk).kind;
        if std::array::IntoIter::new(kinds).any(|it| it == k) {
            self.bump_to(tk);
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn skip_newlines(&mut self) {
        while self.at(T![newline]) {
            self.do_bump(T![newline], 1);
        }
    }

    #[inline(always)]
    fn unexpected(&mut self) {
        let current = self.current();
        if current != EOF {
            self.do_bump(current, 1);
        }

        self.error(format!("Unexpected {:?}", current));
    }

    #[inline(always)]
    fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }

    #[inline(always)]
    fn push_event(&mut self, event: Event) {
        self.events.push(event)
    }
}
///
pub fn parse(text: &str) -> (rowan::GreenNode, Vec<(ParseError, TextSize)>) {
    let lexer = Lexer::new(text);
    let tokens: Vec<_> = lexer
        .map(|(kind, span)| Token {
            syntax_kind: kind,
            len: span.text_range.len(),
        })
        .collect();
    let mut lexer = LexerWrap::new(&tokens);
    let mut sink = TextTreeSink::new(text, &tokens);

    parse_to_sink(&mut lexer, &mut sink);
    sink.finish()
}

/// parses `text`
pub fn parse_to_sink(source: &mut dyn TokenSource, sink: &mut dyn TreeSink) {
    let mut p = Parser {
        source,
        events: vec![],
    };
    p.parse();
    event::process(sink, p.events);
}

#[inline]
fn parse_lang_item(p: &mut Parser) {
    p.skip_newlines();
    match p.current() {
        T![struct] => parse_struct_decl(p),
        _ => parse_statement(p),
    }
}

fn parse_statement(p: &mut Parser) {
    let tok = p.current();

    match tok {
        T!['(']
        | T!['[']
        | T!['{']
        | T![+]
        | T![-]
        | T![not]
        | T![true]
        | T![false]
        | T![int_number]
        | T![float_number]
        | T![ident]
        | T![str]
        | T![multiline_str]
        | T![if] => {
            let statement_marker = p.start();
            parse_expr(p);
            // test assignment
            // a = b

            // test unit_to_unit_assignment
            // () = ()
            if p.at_any([T![+=], T![-=], T![*=], T![/=], T![%=], T![=]]) {
                // consume the `=` / `+=` / ...
                p.bump_any();

                // test assignment_expr_on_next_line
                // x =
                //     1

                parse_expr(p);

                p.require_newline();
                statement_marker.complete(p, ASSIGNMENT_STMT);
            } else {
                // test freestanding_expr
                // 1

                p.require_newline();
                statement_marker.complete(p, EXPR_EVAL_STMT);
            }
        }
        T![let] => {
            parse_declaration(p);
        }
        T![foreach] => {
            parse_foreach(p);
        }
        T![continue] | T![break] | T![return] => {
            parse_control_stmt(p);
        }
        _ => {
            p.unexpected();
        }
    }
}

fn parse_expr(p: &mut Parser) {
    // test precedence_parsing
    // x = 1 + 2 * 3 % - 4 ( 5 )

    parse_precedence_8_expr(p);
}

fn parse_tuple_expr(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    assert!(is_tuple_expr_start(p));

    parse_tt(p, TUPLE_EXPR, T!['('], Some(T![,]), T![')'], parse_expr)
}

fn is_tuple_expr_start(p: &mut Parser) -> bool {
    p.at(T!['('])
}

fn parse_array_expr(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    assert!(is_array_expr_start(p));

    parse_tt(p, ARRAY_LIT_EXPR, T!['['], Some(T![,]), T![']'], parse_expr)
}

fn is_array_expr_start(p: &mut Parser) -> bool {
    p.at(T!['['])
}

fn parse_primary(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();

    let mk = p.start();

    if is_array_expr_start(p) {
        parse_array_expr(p);
    } else if is_tuple_expr_start(p) {
        parse_tuple_expr(p);
    } else if is_conditional_start(p) {
        // test if_condition_in_expr
        // a = if b {} else {}

        // test if_condition_in_expr_stretched
        // a = if b
        // {
        //
        // }
        //
        // else
        // {
        //
        //
        // }
        parse_conditional(p);
    } else if is_expr_block_start(p) {
        parse_expr_block(p);
    } else if p.at_any([T![int_number], T![float_number], T![ident]]) {
        p.bump_any();
    } else if is_string_lit(p) {
        parse_string(p);
    } else {
        p.unexpected();
    }

    mk.complete(p, PRIMARY_EXPR)
}

fn is_string_lit(p: &mut Parser) -> bool {
    p.at_any([T![str], T![multiline_str]])
}

fn parse_string(p: &mut Parser) {
    assert!(is_string_lit(p));
    p.bump_any();
}

fn parse_tt(
    p: &mut Parser,
    outer_kind: SyntaxKind,
    start_tok: SyntaxKind,
    separator: Option<SyntaxKind>,
    end_tok: SyntaxKind,
    mut f: impl FnMut(&mut Parser),
) -> CompletedMarker {
    let marker = p.start();
    p.bump(start_tok);

    p.skip_newlines();
    while !p.at_any([EOF, end_tok]) {
        f(p);

        if let Some(separator) = separator {
            p.skip_newlines();
            if p.eat(separator) {
            } else if !p.at(end_tok) {
                p.error(format!(
                    "expected {:?} or {:?}, got {:?}",
                    separator,
                    end_tok,
                    p.current()
                ));
            }
        }
        p.skip_newlines();
    }

    p.expect(end_tok);
    marker.complete(p, outer_kind)
}

fn parse_precedence_1_expr(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    let mut marker = parse_primary(p);

    // test index_on_second_line_is_array_lit_expr
    // x = a
    // [1]

    while p.at_any([T!['('], T!['[']]) {
        if p.at(T!['(']) {
            let new_marker = marker.precede(p);
            marker = parse_f_call(p, new_marker);
        } else if p.at(T!['[']) {
            let new_marker = marker.precede(p);
            marker = parse_index_expr(p, new_marker);
        }
    }

    marker
}

fn parse_f_call(p: &mut Parser, marker: Marker) -> CompletedMarker {
    // test simple_function_call
    // x = f(1, 2, a = b)

    // test function_call
    // x = f(1, 2, a = b)

    // test named_args_only_function_call
    // x = f(a = b, c = d)

    // test err func_call_on_second_line
    // x = f
    // (1, 2, a = b)

    parse_tt(p, FN_CALL_ARGS, T!['('], Some(T![,]), T![')'], parse_farg);
    marker.complete(p, FN_CALL_EXPR)
}

fn parse_farg(p: &mut Parser) {
    p.skip_newlines();
    if is_kexpr_start(p) {
        parse_kexpr(p)
    } else {
        parse_expr(p)
    }
}

fn parse_kexpr(p: &mut Parser) {
    p.skip_newlines();
    assert!(is_kexpr_start(p));
    let marker = p.start();

    p.bump(T![ident]);
    p.bump(T![=]);

    parse_expr(p);

    marker.complete(p, K_EXPR);
}

fn is_kexpr_start(p: &mut Parser) -> bool {
    p.at(T![ident]) && p.nth(1) == T![=]
}

fn parse_index_expr(p: &mut Parser, marker: Marker) -> CompletedMarker {
    let brackets_marker = p.start();
    p.bump(T!['[']);
    parse_expr(p); // expr
    p.expect(T![']']);
    brackets_marker.complete(p, INDEX_EXPR_BRACKETS);
    marker.complete(p, INDEX_EXPR)
}

fn parse_precedence_2_expr(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    if p.at_any([T![+], T![-]]) {
        let marker = p.start();
        p.bump_any();
        parse_precedence_2_expr(p);
        marker.complete(p, PREFIX_UNARY_EXPR)
    } else {
        parse_precedence_1_expr(p)
    }
}

fn parse_infix_binop<const N: usize>(
    p: &mut Parser,
    ops: [SyntaxKind; N],
    mut lower: impl FnMut(&mut Parser) -> CompletedMarker,
) -> CompletedMarker {
    p.skip_newlines();
    let mut completed = lower(p);

    // test expr_with_binary_infix_operators_on_same_line_and_second_operand_on_second_line
    // x = 1 +
    // 2 +
    // f(
    //      4
    // )
    // y = 3 *
    // 6 %
    // 78

    // test expr_with_binary_infix_operators_on_next_line
    // x = 1
    // + 2
    // + f(
    //      4
    // )
    // y = 3
    // * 6
    // % 78

    while p.bump_to_if_next_non_newline_is_any(ops) {
        let prec = completed.precede(p);
        p.bump_any();
        lower(p);

        completed = prec.complete(p, BIN_EXPR);
    }

    completed
}

fn parse_precedence_3_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![*], T![/], T![%]], parse_precedence_2_expr)
}

fn parse_precedence_4_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![+], T![-]], parse_precedence_3_expr)
}

fn parse_precedence_5_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![<], T![<=], T![>], T![>=]], parse_precedence_4_expr)
}

fn parse_precedence_6_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![==], T![!=]], parse_precedence_5_expr)
}

fn parse_precedence_7_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![&&]], parse_precedence_6_expr)
}

fn parse_precedence_8_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![||]], parse_precedence_7_expr)
}

fn parse_expr_block(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    parse_tt(p, BLOCK, T!['{'], None, T!['}'], parse_statement)
}

fn is_expr_block_start(p: &mut Parser) -> bool {
    p.at(T!['{'])
}

fn parse_declaration(p: &mut Parser) -> CompletedMarker {
    // test declaration
    // let a = b

    // test var_declaration_with_proper_expr_as_value
    // let x = 1
    p.skip_newlines();
    let mk = p.start();
    p.expect(T![let]);
    p.expect(T![ident]);

    p.expect(T![=]);

    parse_expr(p);

    p.require_newline();

    mk.complete(p, DECLARATION_STMT)
}

fn is_conditional_start(p: &mut Parser) -> bool {
    p.at(T![if])
}

fn parse_conditional(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();

    // test if_else_condition
    // if a {} else {}

    // test if_else_if_else_condition
    // if a {} else if b {} else {}

    assert!(is_conditional_start(p));

    let marker = p.start();
    parse_conditional_branch(p);
    // test if_else_if_else_condition_stretched
    // if ()
    //
    // {
    //
    // }
    //
    // else if
    // ()
    //
    // {
    //
    // }
    //
    // else {
    //
    // }

    // test if_condition_on_next_line
    // if
    //      1
    // {}
    while p.bump_to_if_next_non_newline_is(T![else]) {
        p.bump(T![else]);
        if p.bump_to_if_next_non_newline_is(T![if]) {
            parse_conditional_branch(p);
        } else {
            parse_expr_block(p);
            break;
        }
    }

    marker.complete(p, CONDITIONAL)
}

fn parse_conditional_branch(p: &mut Parser) {
    p.skip_newlines();

    p.bump(T![if]);

    parse_expr(p);

    parse_expr_block(p);
}

fn parse_foreach(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();

    // test foreach_basic
    // foreach a in b {}

    let marker = p.start();

    p.bump(T![foreach]);
    parse_for_in_expr(p);
    parse_expr_block(p);

    p.require_newline();

    marker.complete(p, FOREACH_STMT)
}

fn parse_for_in_expr(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    parse_expr(p);
    p.expect(T![in]);
    parse_expr(p);
    marker.complete(p, FOREACH_IN_EXPR)
}

fn is_control_stmt(p: &mut Parser) -> bool {
    p.at_any([T![continue], T![break], T![return]])
}

fn parse_control_stmt(p: &mut Parser) -> CompletedMarker {
    // test continue_test
    // continue

    // test break_without_value
    // break

    // test break_with_value
    // break 1

    // test break_with_value_on_new_line
    // break
    //        1

    // test return_without_value
    // return

    // test return_with_value
    // return 1

    // test return_with_value_on_new_line
    // return
    //         1
    p.skip_newlines();
    assert!(is_control_stmt(p));

    let marker = p.start();
    if p.eat(T![continue]) {
        p.require_newline();
        marker.complete(p, CONTINUE_STMT)
    } else if p.eat(T![return]) {
        if !p.at(T![newline]) {
            parse_expr(p);
        }
        p.require_newline();
        marker.complete(p, RETURN_STMT)
    } else if p.eat(T![break]) {
        if !p.at(T![newline]) {
            parse_expr(p);
        }
        p.require_newline();
        marker.complete(p, BREAK_STMT)
    } else {
        unreachable!()
    }
}

fn parse_struct_decl(p: &mut Parser) {
    p.skip_newlines();

    // test empty_struct
    // struct test {}

    // test struct_with_one_field
    // struct test {
    //     i: i32
    // }

    // test struct_with_field_and_generics
    // struct test {
    //     i: i32<a, b>
    // }

    // test err struct_with_field_name_but_no_type
    // struct test {
    //     i
    // }

    let marker = p.start();
    p.bump(T![struct]);
    p.expect(T![ident]);
    p.skip_newlines();
    parse_tt(
        p,
        STRUCT_FIELD_LIST,
        T!['{'],
        None,
        T!['}'],
        parse_struct_field,
    );
    marker.complete(p, STRUCT_DECL);
    p.require_newline();
}

fn parse_struct_field(p: &mut Parser) {
    let marker = p.start();
    p.expect(T![ident]);
    p.expect(T![:]);
    parse_type_ref(p);
    p.require_newline();
    marker.complete(p, STRUCT_FIELD);
}

fn parse_type_ref(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    p.expect(T![ident]);
    if p.at(T![<]) {
        parse_type_ref_generics(p);
    }
    marker.complete(p, TYPE_REF)
}

fn parse_type_ref_generics(p: &mut Parser) {
    if !p.at(T![<]) {
        return;
    }
    parse_tt(p, GENERIC_PARAMS, T![<], Some(T![,]), T![>], |p| {
        parse_type_ref(p);
    });
}
