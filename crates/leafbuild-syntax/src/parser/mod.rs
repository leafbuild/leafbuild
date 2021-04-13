//! # The parser code
//! Also see [the syntax reference](https://leafbuild.github.io/leafbuild/dev/syntax_ref.html).

use std::fmt;
use std::ops::{Deref, DerefMut, Range};

use rowan::{Checkpoint, GreenNodeBuilder, TextRange, TextSize};

use crate::lexer::Lexer;
use crate::SyntaxKind::{self, *};
use crate::T;
use leafbuild_stdx::{CopyTo, TakeIfUnless};

use self::{
    error::ParseError,
    event::Event,
    marker::{CompletedMarker, Marker},
    token_source::TokenSource,
};

mod error;
mod event;
mod marker;
mod token_source;
mod tree_sink;

///
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Span {
    text_range: TextRange,
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
        write!(f, "{:?}", self.text_range)
    }
}

struct Parser<'ts> {
    /// tokens
    source: &'ts mut dyn TokenSource,
    events: Vec<Event>,
}

#[allow(clippy::inline_always)]
impl<'ts> Parser<'ts> {
    fn parse(mut self) -> Parse {
        self.parse_node(ROOT, |p| {
            p.bump_raw_to_meaningful();
            loop {
                match parse_lang_item(p) {
                    ParseResult::Eof => break,
                    ParseResult::Ok => {}
                    ParseResult::Incomplete => {
                        p.errors.push(("incomplete".into(), Span::default()))
                    }
                    ParseResult::Error(err, span) => {
                        p.errors.push((err, span));
                        break;
                    }
                    Err(ParseError::UnexpectedToken(tk, span)) => {
                        p.errors.push((format!("unexpected `{}`", tk), span));
                        break;
                    }
                    Err(ParseError::ExpectedToken(tk, span, found)) => {
                        p.errors.push((
                            format!("expected token {}, found token {}", tk, found),
                            span,
                        ));
                        break;
                    }
                    Err(ParseError::ExpectedTokens(tokens, span)) => {
                        p.errors
                            .push((format!("expected one of {{{}}}", tokens.join(", ")), span));
                        break;
                    }
                }
            }

            p.bump_raw_to(p.tokens.len());

            Ok(())
        })
        .unwrap();

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    #[inline(always)]
    fn at(&self, kind: SyntaxKind) -> bool {
        self.current().is(kind)
    }

    #[inline(always)]
    fn at_any(&self, kinds: &[SyntaxKind]) -> bool {
        self.current().is_any(kinds)
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
        self.source.bump();
        self.push_event(Event::Token { kind, n_raw_tokens })
    }

    fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(format!("expected {:?}", kind));
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
    fn current_span(&self) -> Span {
        self.source.current().span
    }

    #[inline(always)]
    fn require_newline(&mut self) {
        self.expect(NEWLINE)
    }

    #[inline(always)]
    fn has_newline(&self) -> bool {
        self.at(NEWLINE)
    }

    #[inline(always)]
    fn skip_newlines(&mut self) {
        while self.at(NEWLINE) {
            self.do_bump(NEWLINE, 1);
        }
    }

    #[inline(always)]
    fn unexpected(&mut self) {
        let current = self.current();
        self.do_bump(current, 1);

        self.error(format!("Unexpected {}", current));
    }

    #[inline(always)]
    fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }
}

/// parses `text`
#[must_use]
pub fn parse(text: &str) -> Parse {
    let lexer = Lexer::new(text);
    let tokens: Vec<(SyntaxKind, &str, Span)> = lexer.collect();
    let meaningful = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, (it, _, _))| (*it, index).take_if(|(kind, _)| kind.is_meaningful()))
        .collect();
    Parser {
        builder: GreenNodeBuilder::new(),
        errors: vec![],
        index: 0,
        tokens,
        meaningful,
        meaningful_index: 0,
    }
    .parse()
}

#[inline]
fn parse_lang_item(p: &mut Parser) {
    p.skip_newlines();
    match p.current() {
        Some(STRUCT_KW) => parse_struct_decl(p),
        _ => parse_statement(p),
    }
}

fn parse_statement(p: &mut Parser) {
    let tok = p.current().ok_or(ParseError::Eof)?;
    match tok {
        T!['(']
        | T!['[']
        | T!['{']
        | T![+]
        | T![-]
        | T![not]
        | T![true]
        | T![false]
        | T![NumLit]
        | T![Id]
        | T![Str]
        | T![MultilineStr]
        | T![if] => {
            let assingment_marker = p.start();
            parse_expr(p)?;
            // test assignment
            // a = b

            // test unit_to_unit_assignment
            // () = ()
            if p.at_any(&[T![+=], T![-=], T![*=], T![/=], T![%=], T![=]]) {
                // consume the `=` / `+=` / ...
                p.bump_any();

                // test assignment_expr_on_next_line
                // x =
                //     1

                parse_expr(p)?;

                p.require_newline()?;
                assingment_marker.complete(p, Assignment);
            } else {
                // test freestanding_expr
                // 1

                p.require_newline()?;
                assingment_marker.abandon(p);
            }
        }
        LET_KW => parse_declaration(p),
        FOREACH_KW => parse_foreach(p),
        CONTINUE_KW | BREAK_KW | RETURN_KW => parse_control_stmt(p),
        T![')']
        | T![']']
        | T!['}']
        | T![+=]
        | T![-=]
        | T![*=]
        | T![/=]
        | T![%=]
        | T![*]
        | T![/]
        | T![%]
        | T![==]
        | T![>=]
        | T![>]
        | T![<=]
        | T![<]
        | T![!=]
        | T![=]
        | T![.]
        | T![:]
        | T![?]
        | T![;]
        | T![,]
        | T![and]
        | T![or]
        | T![in]
        | T![fn]
        | T![else]
        | ERROR => {
            // FIXME: error
        }
        _ => unreachable!(),
    }
}

fn parse_expr(p: &mut Parser) {
    p.skip_newlines();

    // test precedence_parsing
    // x = 1 + 2 * 3 % - 4 ( 5 )

    p.parse_node(Expr, parse_precedence_8_expr)
}

fn parse_tuple_expr(p: &mut Parser) {
    p.skip_newlines();
    assert!(is_tuple_expr_start(p));

    parse_tt(p, TupleExpr, T!['['], Some(T![,]), T![']'], parse_expr)
}

fn is_tuple_expr_start(p: &mut Parser) -> bool {
    p.at(L_PAREN)
}

fn parse_array_expr(p: &mut Parser) {
    p.skip_newlines();
    assert!(is_array_expr_start(p));

    parse_tt(p, ArrayLitExpr, T!['['], Some(T![,]), T![']'], parse_expr)
}

fn is_array_expr_start(p: &mut Parser) -> bool {
    p.at(T!['['])
}

fn parse_primary(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    p.parse_node(PrimaryExpr, |p| {
        if is_array_expr_start(p) {
            parse_array_expr(p)
        } else if is_tuple_expr_start(p) {
            parse_tuple_expr(p)
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
            parse_conditional(p)
        } else if is_expr_block_start(p) {
            parse_expr_block(p)
        } else if p.at_any(&[T![NumLit], T![Id]]) {
            p.bump_last();
        } else if is_string_lit(p) {
            parse_string(p)
        } else {
            p.unexpected();
        }
    })
}

fn is_string_lit(p: &mut Parser) -> bool {
    p.at_any(&[T![Str], T![MultilineStr]])
}

fn parse_string(p: &mut Parser) {
    assert!(is_string_lit(p));
    p.bump_any()
}

fn parse_tt(
    p: &mut Parser,
    outer_kind: SyntaxKind,
    start_tok: SyntaxKind,
    separator: Option<SyntaxKind>,
    end_tok: SyntaxKind,
    mut f: impl FnMut(&mut Parser),
) {
    let marker = p.start();
    p.bump(start_tok);

    while !p.at(EOF) && !p.at(end_tok) {
        f(p);

        if let Some(separator) = separator {
            if p.at(separator) {
                p.bump(separator);
            } else if !p.at(end_tok) {
                p.error(format!("expected {:?} or {:?}", separator, end_tok));
            }
        }
    }

    p.expect(end_tok);
    marker.complete(p, outer_kind);
}

fn parse_precedence_1_expr(p: &mut Parser) {
    p.node_start_skip_newlines();
    let mut marker = parse_primary(p);

    // test index_on_second_line_is_array_lit_expr
    // x = a
    // [1]

    while p.at_any(&[T!['('], T!['[']]) {
        if p.at(T!['(']) {
            let new_marker = marker.precede(p);
            marker = parse_f_call(p, new_marker);
        } else if p.at(T!['[']) {
            let new_marker = marker.precede(p);
            marker = parse_index_expr(p, new_marker);
        }
    }
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

    parse_tt(p, FnCallArgsList, T!['('], Some(T![,]), T![')'], parse_farg);
    marker.complete(p, FnCallExpr)
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

    p.bump(T![Id]);
    p.bump(T![=]);

    parse_expr(p);
}

fn is_kexpr_start(p: &mut Parser) -> bool {
    p.node_start_skip_newlines();
    p.at(T![Id]) && p.nth(1) == T![=]
}

fn parse_index_expr(p: &mut Parser, marker: Marker) -> CompletedMarker {
    p.node_start_skip_newlines();
    assert!(p.current().is(L_BRACKET));
    let expr_marker = p.start();
    let brackets_marker = p.start();
    p.bump(T!['{']);
    parse_expr(p); // expr
    p.expect(T!['}']);
    brackets_marker.complete(p, IndexExprBrackets);
    expr_marker.complete(p, IndexExpr)
}

fn parse_precedence_2_expr(p: &mut Parser) {
    p.skip_newlines();
    if p.at_any(&[T![+], T![-]]) {
        let marker = p.start();
        p.bump_any();
        marker.complete(p, PrefixUnaryOpExpr);
    } else {
        parse_precedence_1_expr(p)
    }
}

fn parse_infix_binop(p: &mut Parser, ops: &[SyntaxKind], mut lower: impl FnMut(&mut Parser)) {
    p.skip_newlines();
    let marker = p.start();
    lower(p);

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

    while p.at_any(ops) {
        p.bump_any();
        lower(p);
    }

    Ok(())
}

fn parse_precedence_3_expr(p: &mut Parser) {
    parse_infix_binop(p, &[T![*], T![/], T![%]], parse_precedence_2_expr)
}

fn parse_precedence_4_expr(p: &mut Parser) {
    parse_infix_binop(p, &[T![+], T![-]], parse_precedence_3_expr)
}

fn parse_precedence_5_expr(p: &mut Parser) {
    parse_infix_binop(p, &[T![<], T![<=], T![>], T![>=]], parse_precedence_4_expr)
}

fn parse_precedence_6_expr(p: &mut Parser) {
    parse_infix_binop(p, &[T![==], T![!=]], parse_precedence_5_expr)
}

fn parse_precedence_7_expr(p: &mut Parser) {
    parse_infix_binop(p, &[T![and]], parse_precedence_6_expr)
}

fn parse_precedence_8_expr(p: &mut Parser) {
    parse_infix_binop(p, &[T![or]], parse_precedence_7_expr)
}

fn parse_expr_block(p: &mut Parser) {
    parse_tt(p, ExprBlock, T!['{'], None, T!['}'], parse_statement)
}

fn is_expr_block_start(p: &mut Parser) -> bool {
    p.current().is(T!['{'])
}

fn parse_declaration(p: &mut Parser) {
    // test declaration
    // let a = b

    // test var_declaration_with_proper_expr_as_value
    // let x = 1
    p.skip_newlines();
    let mk = p.start();
    p.expect(T![let]);
    p.expect(T![Id]);

    p.expect(T![=]);

    parse_expr(p);

    p.require_newline();

    mk.complete(p, Declaration);
}

fn is_conditional_start(p: &mut Parser) -> bool {
    p.at(T![if])
}

fn parse_conditional(p: &mut Parser) {
    p.skip_newlines();

    // test if_else_condition
    // if a {} else {}

    // test if_else_if_else_condition
    // if a {} else if b {} else {}

    assert!(is_conditional_start(p));

    p.parse_node(Conditional, |p| {
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
        while p.next_non_newline().copy_to(&mut tk).is(T![else]) {
            p.bump_to(tk);
            p.bump(T![else]);
            if p.next_non_newline().copy_to(&mut tk).is(T![if]) {
                parse_conditional_branch(p).map_incomplete()?;
            } else {
                parse_expr_block(p).map_incomplete()?;
                break;
            }
        }

        Ok(())
    })
}

fn parse_conditional_branch(p: &mut Parser) -> ParseResult {
    p.node_start_skip_newlines();
    assert!(p.current().is(IF_KW));
    p.parse_node(ConditionalBranch, |p| {
        // consume the IF_KW
        p.bump();

        parse_expr(p).map_incomplete()?;

        parse_expr_block(p).map_incomplete()?;

        Ok(())
    })
}

fn parse_foreach(p: &mut Parser) -> ParseResult {
    p.node_start_skip_newlines();

    // test foreach_basic
    // foreach a in b {}

    assert!(p.current().is(FOREACH_KW));
    p.parse_node(Foreach, |p| {
        p.bump(); // FOREACH_KW
        parse_expr(p).map_incomplete()?;
        p.parse_single_tok(IN_KW)?;
        parse_expr(p).map_incomplete()?;
        parse_expr_block(p).map_incomplete()?;
        Ok(())
    })
}

fn parse_control_stmt(p: &mut Parser) -> ParseResult {
    p.node_start_skip_newlines();
    p.parse_node(ControlStatement, |p| match p.current() {
        Some(CONTINUE_KW) => {
            p.bump();
            p.require_newline()?;
            Ok(())
        }

        Some(RETURN_KW) | Some(BREAK_KW) => {
            p.bump();
            if p.current().isnt(NEWLINE) {
                parse_expr(p)?;
            }

            p.require_newline()?;

            Ok(())
        }

        Some(thing) => thing.as_unexpected_token(p.current_span()),
        None => Err(ParseError::Incomplete),
    })
}

fn parse_struct_decl(p: &mut Parser) -> ParseResult {
    p.node_start_skip_newlines();

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

    p.parse_node(StructDecl, |p| {
        p.parse_single_tok(STRUCT_KW).map_incomplete()?;
        p.parse_single_tok(ID).map_incomplete()?;
        p.node_start_skip_newlines();
        parse_tt(
            p,
            StructFieldList,
            L_BRACE,
            None,
            R_BRACE,
            parse_struct_field,
        )?;
        Ok(())
    })
}

fn parse_struct_field(p: &mut Parser) -> ParseResult {
    p.parse_node(StructField, |p| {
        p.parse_single_tok(ID).map_incomplete()?;
        p.parse_single_tok(COLON).map_incomplete()?;
        parse_type_ref(p).map_incomplete()?;
        p.require_newline()?;
        Ok(())
    })
}

fn parse_type_ref(p: &mut Parser) -> ParseResult {
    p.parse_node(TypeRef, |p| {
        p.parse_single_tok(ID).map_incomplete()?;
        if p.at(LESS) {
            parse_type_ref_generics(p)?;
        }

        Ok(())
    })
}

fn parse_type_ref_generics(p: &mut Parser) -> ParseResult {
    parse_tt(
        p,
        TypeRefGenerics,
        LESS,
        Some(COMMA),
        GREATER,
        parse_type_ref,
    )
}
