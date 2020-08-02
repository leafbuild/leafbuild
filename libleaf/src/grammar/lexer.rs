pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct TokLoc {
    begin: usize,
    end: usize,
}

impl TokLoc {
    pub(crate) fn as_rng(&self) -> Range<usize> {
        self.begin..self.end
    }

    #[inline]
    pub(crate) fn get_begin(&self) -> usize {
        self.begin
    }

    #[inline]
    pub(crate) fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Clone, Debug)]
pub enum Tok {
    Newline,
    Number(i32, TokLoc),
    Identifier(String, TokLoc),
    Str(String, TokLoc),

    Add(TokLoc),
    Sub(TokLoc),
    Mul(TokLoc),
    Div(TokLoc),
    Mod(TokLoc),

    AddEq(TokLoc),
    SubEq(TokLoc),
    MulEq(TokLoc),
    DivEq(TokLoc),
    ModEq(TokLoc),
    Eq(TokLoc),

    POPEN(TokLoc),
    PCLOSE(TokLoc),
    Colon(TokLoc),
    Comma(TokLoc),
    Dot(TokLoc),
    Let(TokLoc),
}

impl Display for Tok {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(Debug)]
pub enum LexicalError {
    UnrecognizedToken { location: usize },
    StringStartedButNotEnded { start_loc: usize },
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

use itertools::Itertools;
use std::iter::Peekable;
use std::ops::Range;
use std::str::CharIndices;

pub struct Lexer<'input> {
    chars: Peekable<CharIndices<'input>>,
    input: &'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
        }
    }

    fn parse_identifier(
        &mut self,
        initial_position: usize,
        initial_letter: char,
    ) -> Result<(usize, Tok, usize), LexicalError> {
        let result: String;
        let mut next_position: usize = initial_position + 1;
        result = format!(
            "{}{}",
            initial_letter,
            self.chars
                .peeking_take_while(|(pos, chr)| -> bool {
                    next_position = *pos;
                    chr.is_ascii_alphanumeric() || *chr == '_'
                })
                .map(|(_pos, chr)| chr)
                .collect::<String>()
        );
        let loc = TokLoc {
            begin: initial_position,
            end: next_position,
        };

        // keywords are checked here

        Ok((
            initial_position,
            match &result as &str {
                "let" => Tok::Let(loc),
                _ => Tok::Identifier(result, loc),
            },
            next_position,
        ))
    }

    fn parse_number(
        &mut self,
        initial_position: usize,
        initial_char: char,
    ) -> Result<(usize, Tok, usize), LexicalError> {
        match self.chars.peek() {
            None => Ok((
                initial_position,
                Tok::Number(
                    Self::decdigit_value(initial_char),
                    TokLoc {
                        begin: initial_position,
                        end: initial_position + 1,
                    },
                ),
                initial_position + 1,
            )),
            Some((_i, chr)) => {
                if initial_char == '0' {
                    if *chr == 'x' {
                        // parse as hex
                        self.chars.next().unwrap(); // take the 'x' out of the stream
                        let mut num = 0;
                        let end_position;
                        loop {
                            match self.chars.peek() {
                                Some((_pos, character)) if character.is_ascii_hexdigit() => {
                                    num = num * 16 + Self::hexdigit_value(*character);
                                    self.chars.next();
                                }
                                None => {
                                    end_position = self.input.len();
                                    break;
                                }
                                Some((pos, _)) => {
                                    end_position = *pos;
                                    break;
                                }
                            };
                        }
                        Ok((
                            initial_position,
                            Tok::Number(
                                num,
                                TokLoc {
                                    begin: initial_position,
                                    end: end_position,
                                },
                            ),
                            end_position,
                        ))
                    } else {
                        // parse as oct
                        let mut num = 0;
                        let end_position;
                        loop {
                            match self.chars.peek() {
                                Some((_pos, character)) if character.is_digit(8) => {
                                    num = num * 8 + Self::octdigit_value(*character);
                                    self.chars.next();
                                }
                                None => {
                                    end_position = self.input.len();
                                    break;
                                }
                                Some((pos, _)) => {
                                    end_position = *pos;
                                    break;
                                }
                            };
                        }
                        Ok((
                            initial_position,
                            Tok::Number(
                                num,
                                TokLoc {
                                    begin: initial_position,
                                    end: end_position,
                                },
                            ),
                            end_position,
                        ))
                    }
                } else {
                    let mut num = Self::decdigit_value(initial_char);
                    let end_position;
                    loop {
                        match self.chars.peek() {
                            Some((_pos, character)) if character.is_ascii_digit() => {
                                num = num * 10 + Self::decdigit_value(*character);
                                self.chars.next();
                            }
                            None => {
                                end_position = self.input.len();
                                break;
                            }
                            Some((pos, _)) => {
                                end_position = *pos;
                                break;
                            }
                        };
                    }
                    Ok((
                        initial_position,
                        Tok::Number(
                            num,
                            TokLoc {
                                begin: initial_position,
                                end: end_position,
                            },
                        ),
                        end_position,
                    ))
                }
            }
        }
    }

    fn parse_string(
        &mut self,
        initial_position: usize,
    ) -> Result<(usize, Tok, usize), LexicalError> {
        // we know we have a '\'' already from the self.chars.next() in the match in the iterator implementation
        match self.chars.peek() {
            Some((_, '\'')) => {
                self.chars.next();
                match self.chars.peek() {
                    Some((_, '\'')) => {
                        // parse multiline string
                        self.chars.next();
                        let mut prev = ['0', '0'];
                        let mut s: String = self
                            .chars
                            .peeking_take_while(|(_, chr)| {
                                let r = *chr != '\'' || prev[0] != '\'' || prev[1] != '\'';
                                prev[0] = prev[1];
                                prev[1] = *chr;
                                r
                            })
                            .map(|(_, chr)| chr)
                            .collect();
                        let (last_single_quote_index, _) = match self.chars.next() /* try to take the last ' out of the iterator*/{
                            Some(x) => x,
                            None => {return Err(LexicalError::StringStartedButNotEnded {start_loc: initial_position})}
                        };

                        // and remove the last 2 single quotes
                        s.pop();
                        s.pop();
                        Ok((
                            initial_position,
                            Tok::Str(
                                s,
                                TokLoc {
                                    begin: initial_position,
                                    end: last_single_quote_index + 1,
                                },
                            ),
                            last_single_quote_index + 1,
                        ))
                    }
                    _ => Ok((
                        initial_position,
                        Tok::Str(
                            "".to_string(),
                            TokLoc {
                                begin: initial_position,
                                end: initial_position + 2,
                            },
                        ),
                        initial_position + 2,
                    )),
                }
            }
            Some((_, _)) => {
                // parse simple ' ... ' string
                let mut last_index = 0;
                let mut prev_chr: char = '0'; // doesn't really matter
                let mut prev_escaped: bool = false;
                let s: String = Self::str_escape(
                    self.chars
                        .peeking_take_while(|(_, chr)| {
                            if !(*chr != '\'' || prev_chr == '\\' && prev_escaped) {
                                // *chr == '\'' && (prev_chr == '\\' && !prev_escaped || prev_chr != '\\'), but clippy changed it to this
                                false
                            } else {
                                if *chr == '\\' {
                                    prev_escaped = !prev_escaped;
                                } else {
                                    prev_escaped = false;
                                }
                                prev_chr = *chr;
                                true
                            }
                        })
                        .map(|(index, chr)| {
                            last_index = index;
                            chr
                        }),
                );
                match self.chars.next()  /* try to take the '\'' out of the iterator*/{
                    Some((_, '\'')) => {},
                    _ => {return Err(LexicalError::StringStartedButNotEnded {start_loc: initial_position})}
                }
                Ok((
                    initial_position,
                    Tok::Str(
                        s,
                        TokLoc {
                            begin: initial_position,
                            end: last_index + 2,
                        },
                    ),
                    last_index + 2,
                ))
            }
            None => Err(LexicalError::StringStartedButNotEnded {
                start_loc: initial_position,
            }),
        }
    }

    #[inline]
    fn octdigit_value(chr: char) -> i32 {
        (chr as u8 - b'0') as i32
    }

    #[inline]
    fn decdigit_value(chr: char) -> i32 {
        (chr as u8 - b'0') as i32
    }

    #[inline]
    fn hexdigit_value(chr: char) -> i32 {
        let chr = chr as u8;
        (match chr {
            b'0'..=b'9' => chr - b'0',
            b'a'..=b'f' => chr - b'a',
            b'A'..=b'F' => chr - b'A',
            _ => 0,
        }) as i32
    }

    #[inline]
    fn str_escape<T: Iterator<Item = char>>(s: T) -> String {
        let mut prev: char = '0';
        let mut prev_escaped: bool = true;
        let mut result = String::new();
        for (_, chr) in s.enumerate() {
            if chr == '\\' {
                if prev == '\\' && !prev_escaped {
                    result += &chr.to_string();
                }
                prev_escaped = !prev_escaped;
            } else {
                result += &chr.to_string();
                prev_escaped = true;
            }

            prev = chr;
        }
        result
    }
}

/// A token made up of a single char; the token's name is $name and $i is the position
macro_rules! single_char_token {
    ($name:ident, $i: expr) => {
        Some(Ok((
            $i,
            Tok::$name(TokLoc {
                begin: $i,
                end: $i + 1,
            }),
            $i + 1,
        )))
    };
}

/// When we met a '+', '-', '*', '/' or '%', depending on what the next character is, '=' or not,
/// we will return Tok::$x if it is or Tok::$y if it is not, $slf being self and $i being the
/// index of the '+', '-', '*', '/' or '%', used in <Lexer as Iterator>::next
macro_rules! possibly_assignment_op {
    ($slf:expr, $x:ident, $y:ident, $i:expr) => {
        match $slf.chars.peek() {
            Some((_, '=')) => {
                $slf.chars.next();
                Some(Ok((
                    $i,
                    Tok::$x(TokLoc {
                        begin: $i,
                        end: $i + 2,
                    }),
                    $i + 2,
                )))
            }
            _ => Some(Ok((
                $i,
                Tok::$y(TokLoc {
                    begin: $i,
                    end: $i + 1,
                }),
                $i + 1,
            ))),
        }
    };
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, '\n')) => return Some(Ok((i, Tok::Newline, i + 1))),
                Some((_, chr)) if chr.is_whitespace() => continue,
                Some((i, ':')) => return single_char_token!(Colon, i),
                Some((i, ',')) => return single_char_token!(Comma, i),
                Some((i, '.')) => return single_char_token!(Dot, i),
                Some((i, '(')) => return single_char_token!(POPEN, i),
                Some((i, ')')) => return single_char_token!(PCLOSE, i),
                Some((i, '+')) => return possibly_assignment_op!(self, AddEq, Add, i),
                Some((i, '-')) => return possibly_assignment_op!(self, SubEq, Sub, i),
                Some((i, '*')) => return possibly_assignment_op!(self, MulEq, Mul, i),
                Some((i, '/')) => return possibly_assignment_op!(self, DivEq, Div, i),
                Some((i, '%')) => return possibly_assignment_op!(self, ModEq, Mod, i),
                Some((i, '=')) => return single_char_token!(Eq, i),
                Some((i, chr)) if chr.is_ascii_alphabetic() || chr == '_' => {
                    return Some(self.parse_identifier(i, chr));
                }
                Some((i, chr)) if chr.is_ascii_digit() => return Some(self.parse_number(i, chr)),
                Some((i, chr)) if chr == '\'' => return Some(self.parse_string(i)),
                None => return None, // End of file
                Some((i, _)) => return Some(Err(LexicalError::UnrecognizedToken { location: i })),
            }
        }
    }
}
