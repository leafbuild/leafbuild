//! FIXME: write docs
use rowan::{GreenNode, GreenNodeBuilder};

use crate::lexer::Lexer;
use crate::syntax_kind::SyntaxKind::{self, *};
use leafbuild_core::utils::Let;
use std::ops::Range;

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

use std::fmt;

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
    Error(String, Span),
    UnexpectedToken(String, Span),
    ExpectedToken(String, Span),
    ExpectedTokens(Vec<String>, Span),
}

type ParseResult<T = ()> = std::result::Result<T, ParseError>;

impl Parser {
    fn parse(mut self) -> Parse {
        self.builder.start_node(ROOT.into());

        loop {
            match parse_lang_item(&mut self) {
                Err(ParseError::Eof) => break,
                Ok(()) => {}
                Err(ParseError::Error(err, span)) => {
                    self.errors.push((err, span));
                    break;
                }
                Err(ParseError::UnexpectedToken(tk, span)) => {
                    self.errors.push((format!("unexpected `{}`", tk), span));
                    break;
                }
                Err(ParseError::ExpectedToken(tk, span)) => {
                    self.errors.push((format!("expected token {}", tk), span));
                    break;
                }
                Err(ParseError::ExpectedTokens(tokens, span)) => {
                    self.errors
                        .push((format!("expected one of {{{}}}", tokens.join(", ")), span));
                    break;
                }
            }
        }

        self.skip_trivia();
        self.builder.finish_node();

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

    fn skip_trivia(&mut self) {
        while self
            .current()
            .let_(|it| it.is(WHITESPACE) || it.is(LINE_COMMENT) || it.is(BLOCK_COMMENT))
        {
            self.bump()
        }
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
            self.error()
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
            ));
        }
        Ok(())
    }

    fn parse_newline(&mut self) -> ParseResult {
        self.parse_single_tok(NEWLINE)
    }

    fn lookahead(&mut self) -> Option<SyntaxKind> {
        self.tokens.get(self.index + 1).map(|(kind, _, _)| *kind)
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
    p.skip_trivia();
    let tok = p.current().ok_or(ParseError::Eof)?;
    match tok {
        L_PAREN | L_BRACKET | L_BRACE | PLUS | MINUS | NOT_KW | TRUE_KW | FALSE_KW | NUMBER
        | ID | STRING | MULTILINE_STRING => {
            let assignment_checkpoint = p.builder.checkpoint();
            parse_expr(p)?;
            p.skip_trivia();
            let current = p.current();
            if current.is(PLUS_EQ)
                || current.is(MINUS_EQ)
                || current.is(MUL_EQ)
                || current.is(DIV_EQ)
                || current.is(MOD_EQ)
                || current.is(EQ)
            {
                p.builder
                    .start_node_at(assignment_checkpoint, Assignment.into());
                p.bump();

                parse_expr(p)?;

                p.parse_newline()?;

                p.builder.finish_node();
            }

            Ok(())
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
    p.skip_trivia();

    let current = p.current().ok_or(ParseError::Eof)?;
    match current {
        L_PAREN => parse_tuple_expr(p),
        L_BRACKET => parse_array_expr(p),
        L_BRACE => parse_expr_block(p),

        // R_PAREN | R_BRACKET | R_BRACE => current.as_unexpected_token(),
        _ => current.as_unexpected_token(p.current_span()),
    }
}

fn parse_tuple_expr(p: &mut Parser) -> ParseResult {
    assert!(p.current().is(L_PAREN));

    parse_tt(p, TupleExpr, L_PAREN, COMMA, R_PAREN, parse_expr)
}

fn parse_array_expr(p: &mut Parser) -> ParseResult {
    assert!(p.current().is(L_BRACKET));

    parse_tt(p, ArrayLitExpr, L_BRACKET, COMMA, R_BRACKET, parse_expr)
}

fn parse_tt(
    p: &mut Parser,
    outer_kind: SyntaxKind,
    start_tok: SyntaxKind,
    separator: SyntaxKind,
    end_tok: SyntaxKind,
    mut f: impl FnMut(&mut Parser) -> ParseResult,
) -> ParseResult {
    assert!(p.current().is(start_tok));
    p.builder.start_node(outer_kind.into());

    p.bump();

    p.skip_trivia();

    while p.current().isnt(end_tok) {
        f(p)?;

        p.skip_trivia();

        if !p.bump_if(|it| it.is(separator)) && p.current().isnt(end_tok) {
            p.error();

            return Err(ParseError::ExpectedTokens(
                vec![separator.token_name(), end_tok.token_name()],
                p.current_span(),
            ));
        }

        p.skip_trivia();
    }

    // consume the end token
    p.bump();

    p.builder.finish_node();

    Ok(())
}

fn parse_expr_block(p: &mut Parser) -> ParseResult {
    Ok(())
}

fn parse_declaration(p: &mut Parser) -> ParseResult {
    assert!(p.current().is(LET_KW));
    p.builder.start_node(Declaration.into());
    p.bump();
    p.skip_trivia();

    p.parse_single_tok(ID)?;

    p.skip_trivia();

    p.parse_single_tok(EQ)?;

    p.skip_trivia();

    parse_expr(p)?;

    p.parse_newline()?;

    p.builder.finish_node();
    Ok(())
}

fn parse_conditional(p: &mut Parser) -> ParseResult {
    Ok(())
}

fn parse_foreach(p: &mut Parser) -> ParseResult {
    Ok(())
}

fn parse_control_stmt(p: &mut Parser) -> ParseResult {
    Ok(())
}
