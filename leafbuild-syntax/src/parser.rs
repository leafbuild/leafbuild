//! FIXME: write docs

use std::fmt;
use std::ops::Range;

use rowan::{Checkpoint, GreenNode, GreenNodeBuilder};

use crate::lexer::Lexer;
use crate::syntax_kind::SyntaxKind::{self, *};
use leafbuild_core::utils::{Let, TakeIfUnless};

///
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

///
#[derive(Debug)]
pub struct Parse {
    /// the node
    pub green_node: GreenNode,
    /// errors
    #[allow(unused)]
    pub errors: Vec<(String, Span)>,
}

struct Parser {
    /// tokens
    tokens: Vec<(SyntaxKind, String, Span)>,
    index: usize,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<(String, Span)>,
}

/// `is` helper
trait Is: Sized + Copy {
    fn is(self, kind: SyntaxKind) -> bool;

    fn isnt(self, kind: SyntaxKind) -> bool;

    fn is_any(self, kinds: &[SyntaxKind]) -> bool {
        kinds.iter().any(|&it| self.is(it))
    }
}

impl Is for SyntaxKind {
    fn is(self, kind: SyntaxKind) -> bool {
        self == kind
    }

    fn isnt(self, kind: SyntaxKind) -> bool {
        self != kind
    }
}

impl Is for Option<SyntaxKind> {
    fn is(self, kind: SyntaxKind) -> bool {
        self.map_or(false, |it| it.is(kind))
    }

    fn isnt(self, kind: SyntaxKind) -> bool {
        self.map_or(false, |it| it.isnt(kind))
    }
}

#[derive(Debug, Clone)]
enum ParseError {
    Eof,
    Incomplete,
    Error(String, Span),
    UnexpectedToken(String, Span),
    ExpectedToken(String, Span, String),
    ExpectedTokens(Vec<String>, Span),
}

trait MapIncomplete {
    /// Eof => Incomplete
    fn map_incomplete(self) -> Self;
}

impl MapIncomplete for ParseError {
    fn map_incomplete(self) -> Self {
        match self {
            Self::Eof => Self::Incomplete,
            other => other,
        }
    }
}

impl<T, E: MapIncomplete> MapIncomplete for Result<T, E> {
    fn map_incomplete(self) -> Self {
        self.map_err(MapIncomplete::map_incomplete)
    }
}

trait Trivia: Copy {
    fn is_trivia(self) -> bool;
    fn is_newline(self) -> bool;

    fn is_meaningful(self) -> bool;
}

impl<T: Is> Trivia for T {
    fn is_trivia(self) -> bool {
        self.is_any(&[WHITESPACE, LINE_COMMENT, BLOCK_COMMENT])
    }

    fn is_newline(self) -> bool {
        self.is(NEWLINE)
    }

    fn is_meaningful(self) -> bool {
        !self.is_trivia() && !self.is_newline()
    }
}

type ParseResult<T = ()> = std::result::Result<T, ParseError>;

impl Parser {
    fn parse(mut self) -> Parse {
        self.parse_node(ROOT, |p| {
            p.skip_to_next_meaningful();

            loop {
                match parse_lang_item(p) {
                    Err(ParseError::Eof) => break,
                    Ok(()) => {}
                    Err(ParseError::Incomplete) => {
                        p.errors.push(("incomplete".into(), Span::default()))
                    }
                    Err(ParseError::Error(err, span)) => {
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

            p.skip_to_next_meaningful();
            Ok(())
        })
        .unwrap();

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    /// Advance one token, adding it to the current branch of the tree builder.
    fn bump(&mut self) {
        if let Some((kind, text, _)) = self.tokens.get(self.index) {
            self.builder.token(kind.into(), text.as_str());
            self.index += 1;
        }
    }

    fn bump_if(&mut self, f: impl FnOnce(SyntaxKind) -> bool) -> bool {
        if self.current().map_or(false, f) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn prev(&self) -> Option<SyntaxKind> {
        self.tokens.get(self.index - 1).map(|(kind, _, _)| *kind)
    }

    fn current(&self) -> Option<SyntaxKind> {
        self.tokens.get(self.index).map(|(kind, _, _)| *kind)
    }

    fn current_span(&self) -> Span {
        self.tokens
            .get(self.index)
            .map_or(Span::default(), |(_, _, span)| *span)
    }

    fn next_meaningful(&self) -> Option<SyntaxKind> {
        self.tokens[self.index..]
            .iter()
            .find_map(|(it, _, _)| (*it).take_if(|it| it.is_meaningful()))
    }

    fn next_meaningful_lookahead(&self) -> Option<SyntaxKind> {
        self.tokens[self.index + 1..]
            .iter()
            .find_map(|(it, _, _)| (*it).take_if(|it| it.is_meaningful()))
    }

    fn skip_to_next_meaningful(&mut self) -> Option<SyntaxKind> {
        while let Some((kind, _, _)) = self.tokens.get(self.index) {
            if kind.is_meaningful() {
                return Some(*kind);
            }

            self.index += 1;
        }

        None
    }

    fn skip_trivia(&mut self) {
        while self.current().let_(Trivia::is_trivia) {
            self.bump()
        }
    }

    fn skip_trivia_and_newlines(&mut self) {
        while self.current().let_(|it| it.is_trivia() || it.is_newline()) {
            self.bump()
        }
    }

    fn skip_trivia_and_single_newline(&mut self) -> ParseResult {
        while self.current().is_trivia() {
            self.bump()
        }

        if self.bump_if(|it| it.is(NEWLINE)) {
            Ok(())
        } else {
            Err(ParseError::Incomplete)
        }
    }

    fn next_nontrivia(&self) -> Option<SyntaxKind> {
        self.tokens[self.index..]
            .iter()
            .find_map(|(it, _, _)| (*it).take_unless(|&it| it.is_trivia()))
    }

    fn error(&mut self) {
        self.builder.token(ERROR.into(), "")
    }

    fn parse_single_tok_wrapped(
        &mut self,
        kind: SyntaxKind,
        output_kind: SyntaxKind,
    ) -> ParseResult {
        self.builder.start_node(output_kind.into());
        if !self.bump_if(|it| it.is(kind)) {
            let current = self.current();
            self.errors.push((
                format!("Expected {:?}, got {:?}", kind, current),
                self.current_span(),
            ));
            self.error();
            return self
                .current()
                .unwrap()
                .as_unexpected_token(self.current_span());
        }
        self.builder.finish_node();
        Ok(())
    }

    fn parse_single_tok(&mut self, kind: SyntaxKind) -> ParseResult {
        if !self.bump_if(|it| it.is(kind)) {
            self.error();

            return Err(ParseError::ExpectedToken(
                kind.token_name(),
                self.current_span(),
                self.current().unwrap_or(ERROR).token_name(),
            ));
        }
        Ok(())
    }

    fn parse_newline(&mut self) -> ParseResult {
        self.skip_trivia();
        self.parse_single_tok(NEWLINE)
    }

    fn lookahead(&mut self) -> Option<SyntaxKind> {
        self.tokens.get(self.index + 1).map(|(kind, _, _)| *kind)
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into())
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into())
    }

    fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn finish_node(&mut self) {
        self.builder.finish_node()
    }

    fn parse_node<T>(
        &mut self,
        kind: SyntaxKind,
        f: impl FnOnce(&mut Self) -> ParseResult<T>,
    ) -> ParseResult<T> {
        self.start_node(kind);
        let r = f(self);
        self.finish_node();

        r
    }

    fn parse_node_at<T>(
        &mut self,
        checkpoint: Checkpoint,
        kind: SyntaxKind,
        f: impl FnOnce(&mut Self) -> ParseResult<T>,
    ) -> ParseResult<T> {
        self.start_node_at(checkpoint, kind);
        let r = f(self);
        self.finish_node();

        r
    }
}

/// parses `text`
#[must_use]
pub fn parse(text: &str) -> Parse {
    let lexer = Lexer::new(text);
    let tokens = lexer.collect();
    Parser {
        builder: GreenNodeBuilder::new(),
        errors: vec![],
        index: 0,
        tokens,
    }
    .parse()
}

fn parse_lang_item(p: &mut Parser) -> ParseResult {
    p.skip_trivia();

    parse_statement(p)?;

    Ok(())
}

trait AsUnexpectedToken: Copy {
    fn as_unexpected_token(self, span: Span) -> ParseResult;
}

impl AsUnexpectedToken for SyntaxKind {
    fn as_unexpected_token(self, span: Span) -> ParseResult {
        Err(ParseError::UnexpectedToken(self.token_name(), span))
    }
}

fn parse_statement(p: &mut Parser) -> ParseResult {
    p.skip_to_next_meaningful();
    let tok = p.current().ok_or(ParseError::Eof)?;
    match tok {
        L_PAREN | L_BRACKET | L_BRACE | PLUS | MINUS | NOT_KW | TRUE_KW | FALSE_KW | NUMBER
        | ID | STRING | MULTILINE_STRING => {
            let assignment_checkpoint = p.checkpoint();
            parse_expr(p)?;
            p.skip_trivia();
            if p.current()
                .is_any(&[PLUS_EQ, MINUS_EQ, MUL_EQ, DIV_EQ, MOD_EQ, EQ])
            {
                p.parse_node_at(assignment_checkpoint, Assignment, |p| {
                    // consume the `=` / `+=` / ...
                    p.bump();

                    parse_expr(p)?;

                    p.parse_newline()?;
                    Ok(())
                })
            } else {
                Ok(())
            }
        }
        R_PAREN | R_BRACKET | R_BRACE | PLUS_EQ | MINUS_EQ | MUL_EQ | DIV_EQ | MOD_EQ
        | ASTERISK | SLASH | PERCENT | EQ_EQ | GREATER_EQ | GREATER | LESS_EQ | LESS | NOT_EQ
        | EQ | SHIFT_LEFT | SHIFT_RIGHT | DOT | COLON | QMARK | SEMICOLON | COMMA | TILDE
        | AND_KW | OR_KW | IN_KW | FN_KW | ELSE_KW | ERROR => {
            tok.as_unexpected_token(p.current_span())
        }
        LET_KW => parse_declaration(p),
        IF_KW => parse_conditional(p),
        FOREACH_KW => parse_foreach(p),
        CONTINUE_KW | BREAK_KW | RETURN_KW => parse_control_stmt(p),
        _ => unreachable!(),
    }
}

fn parse_expr(p: &mut Parser) -> ParseResult {
    p.skip_to_next_meaningful();

    let current = p.current().ok_or(ParseError::Eof)?;
    match current {
        L_PAREN => parse_tuple_expr(p),
        L_BRACKET => parse_array_expr(p),
        L_BRACE => parse_expr_block(p),
        IF_KW => parse_conditional(p),

        // R_PAREN | R_BRACKET | R_BRACE => current.as_unexpected_token(),
        _ => current.as_unexpected_token(p.current_span()),
    }
}

fn parse_tuple_expr(p: &mut Parser) -> ParseResult {
    assert!(p.current().is(L_PAREN));

    parse_tt(p, TupleExpr, L_PAREN, Some(COMMA), R_PAREN, parse_expr)
}

fn parse_array_expr(p: &mut Parser) -> ParseResult {
    assert!(p.current().is(L_BRACKET));

    parse_tt(
        p,
        ArrayLitExpr,
        L_BRACKET,
        Some(COMMA),
        R_BRACKET,
        parse_expr,
    )
}

fn parse_tt(
    p: &mut Parser,
    outer_kind: SyntaxKind,
    start_tok: SyntaxKind,
    separator: Option<SyntaxKind>,
    end_tok: SyntaxKind,
    mut f: impl FnMut(&mut Parser) -> ParseResult,
) -> ParseResult {
    assert!(p.current().is(start_tok));
    p.parse_node(outer_kind, move |p| {
        p.bump();

        p.skip_to_next_meaningful();

        while p.current().isnt(end_tok) {
            f(p).map_incomplete()?;

            p.skip_to_next_meaningful();

            if let Some(separator) = separator {
                if !p.bump_if(|it| it.is(separator)) && p.current().isnt(end_tok) {
                    p.error();

                    return Err(ParseError::ExpectedTokens(
                        vec![end_tok.token_name(), separator.token_name()],
                        p.current_span(),
                    ));
                }
                p.skip_to_next_meaningful();
            }
        }

        // consume the end token
        p.bump();

        Ok(())
    })
}

fn parse_expr_block(p: &mut Parser) -> ParseResult {
    p.skip_to_next_meaningful();
    parse_tt(p, ExprBlock, L_BRACE, None, R_BRACE, parse_statement)
}

fn parse_declaration(p: &mut Parser) -> ParseResult {
    assert!(p.current().is(LET_KW));
    p.parse_node(Declaration, move |p| {
        // consume LET_KW
        p.bump();
        // skip whitespace / comments
        p.skip_trivia();

        p.parse_single_tok(ID).map_incomplete()?;

        p.skip_trivia();

        p.parse_single_tok(EQ).map_incomplete()?;

        p.skip_trivia();

        parse_expr(p).map_incomplete()?;

        p.parse_newline().map_incomplete()?;

        Ok(())
    })
}

fn parse_conditional(p: &mut Parser) -> ParseResult {
    assert!(p.current().is(IF_KW));

    p.parse_node(Conditional, move |p| {
        parse_conditional_branch(p).map_incomplete()?;
        while p.next_meaningful().is(ELSE_KW) {
            p.skip_to_next_meaningful();
            p.bump();
            if p.next_meaningful().is(IF_KW) {
                p.skip_to_next_meaningful();
                parse_conditional_branch(p).map_incomplete()?;
            } else {
                p.skip_to_next_meaningful();
                parse_expr_block(p).map_incomplete()?;
                break;
            }
        }

        Ok(())
    })
}

fn parse_conditional_branch(p: &mut Parser) -> ParseResult {
    assert!(p.current().is(IF_KW));
    p.parse_node(ConditionalBranch, move |p| {
        // consume the IF_KW
        p.bump();

        parse_expr(p).map_incomplete()?;

        parse_expr_block(p).map_incomplete()?;

        Ok(())
    })
}

fn parse_foreach(p: &mut Parser) -> ParseResult {
    assert!(p.current().is(FOREACH_KW));
    p.parse_node(Foreach, |p| {
        p.bump(); // FOREACH_KW
        p.skip_trivia();
        parse_expr(p).map_incomplete()?;
        p.skip_trivia();
        p.parse_single_tok(IN_KW)?;
        p.skip_trivia();
        parse_expr(p).map_incomplete()?;
        p.skip_trivia();
        parse_expr_block(p).map_incomplete()?;
        Ok(())
    })
}

fn parse_control_stmt(p: &mut Parser) -> ParseResult {
    p.parse_node(ControlStatement, |p| match p.current() {
        Some(CONTINUE_KW) => {
            p.bump();
            Ok(())
        }

        Some(RETURN_KW) | Some(BREAK_KW) => {
            p.bump();
            if p.next_nontrivia().isnt(NEWLINE) {
                parse_expr(p)?;
            }

            p.skip_trivia_and_single_newline()?;

            Ok(())
        }

        Some(thing) => thing.as_unexpected_token(p.current_span()),
        None => Err(ParseError::Incomplete),
    })
}
