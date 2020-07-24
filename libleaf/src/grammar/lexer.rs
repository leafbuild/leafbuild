pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct TokLoc {
    begin: usize,
    len: usize,
    line: usize,
    char_begin: usize,
}

#[derive(Clone, Debug)]
pub enum Tok {
    Newline,
    Number(i32, TokLoc),
    Identifier(String, TokLoc),

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
}

impl Display for Tok {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(Debug)]
pub enum LexicalError {
    UnrecognizedToken { location: usize },
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

use itertools::Itertools;
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'input> {
    chars: Peekable<CharIndices<'input>>,
    input: &'input str,
    line: usize,
    line_begin: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
            line: 1,
            line_begin: 0,
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
        Ok((
            initial_position,
            Tok::Identifier(
                result,
                TokLoc {
                    begin: initial_position,
                    len: next_position - initial_position,
                    line: self.line,
                    char_begin: initial_position - self.line_begin + 1,
                },
            ),
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
                        len: 1,
                        line: self.line,
                        char_begin: initial_position - self.line_begin + 1,
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
                                    len: end_position - initial_position,
                                    line: self.line,
                                    char_begin: initial_position - self.line_begin + 1,
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
                                    len: end_position - initial_position,
                                    line: self.line,
                                    char_begin: initial_position - self.line_begin + 1,
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
                                len: end_position - initial_position,
                                line: self.line,
                                char_begin: initial_position - self.line_begin + 1,
                            },
                        ),
                        end_position,
                    ))
                }
            }
        }
    }

    fn octdigit_value(chr: char) -> i32 {
        (chr as u8 - b'0') as i32
    }

    fn decdigit_value(chr: char) -> i32 {
        (chr as u8 - b'0') as i32
    }

    fn hexdigit_value(chr: char) -> i32 {
        let chr = chr as u8;
        (match chr {
            b'0'..=b'9' => chr - b'0',
            b'a'..=b'f' => chr - b'a',
            b'A'..=b'F' => chr - b'A',
            _ => 0,
        }) as i32
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, '\n')) => return Some(Ok((i, Tok::Newline, i + 1))),
                Some((_, chr)) if chr.is_whitespace() => continue,
                Some((i, ':')) => {
                    return Some(Ok((
                        i,
                        Tok::Colon(TokLoc {
                            begin: i,
                            len: 1,
                            line: self.line,
                            char_begin: i - self.line_begin + 1,
                        }),
                        i + 1,
                    )))
                }
                Some((i, ',')) => {
                    return Some(Ok((
                        i,
                        Tok::Comma(TokLoc {
                            begin: i,
                            len: 1,
                            line: self.line,
                            char_begin: i - self.line_begin + 1,
                        }),
                        i + 1,
                    )))
                }
                Some((i, '.')) => {
                    return Some(Ok((
                        i,
                        Tok::Dot(TokLoc {
                            begin: i,
                            len: 1,
                            line: self.line,
                            char_begin: i - self.line_begin + 1,
                        }),
                        i + 1,
                    )))
                }
                Some((i, '(')) => {
                    return Some(Ok((
                        i,
                        Tok::POPEN(TokLoc {
                            begin: i,
                            len: 1,
                            line: self.line,
                            char_begin: i - self.line_begin + 1,
                        }),
                        i + 1,
                    )))
                }
                Some((i, ')')) => {
                    return Some(Ok((
                        i,
                        Tok::PCLOSE(TokLoc {
                            begin: i,
                            len: 1,
                            line: self.line,
                            char_begin: i - self.line_begin + 1,
                        }),
                        i + 1,
                    )))
                }
                Some((i, '+')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((
                                i,
                                Tok::AddEq(TokLoc {
                                    begin: i,
                                    len: 2,
                                    line: self.line,
                                    char_begin: i - self.line_begin + 1,
                                }),
                                i + 2,
                            )))
                        }
                        _ => Some(Ok((
                            i,
                            Tok::Add(TokLoc {
                                begin: i,
                                len: 1,
                                line: self.line,
                                char_begin: i - self.line_begin + 1,
                            }),
                            i + 1,
                        ))),
                    }
                }
                Some((i, '-')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((
                                i,
                                Tok::SubEq(TokLoc {
                                    begin: i,
                                    len: 2,
                                    line: self.line,
                                    char_begin: i - self.line_begin + 1,
                                }),
                                i + 2,
                            )))
                        }
                        _ => Some(Ok((
                            i,
                            Tok::Sub(TokLoc {
                                begin: i,
                                len: 1,
                                line: self.line,
                                char_begin: i - self.line_begin + 1,
                            }),
                            i + 1,
                        ))),
                    }
                }
                Some((i, '*')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((
                                i,
                                Tok::MulEq(TokLoc {
                                    begin: i,
                                    len: 2,
                                    line: self.line,
                                    char_begin: i - self.line_begin + 1,
                                }),
                                i + 2,
                            )))
                        }
                        _ => Some(Ok((
                            i,
                            Tok::Mul(TokLoc {
                                begin: i,
                                len: 1,
                                line: self.line,
                                char_begin: i - self.line_begin + 1,
                            }),
                            i + 1,
                        ))),
                    }
                }
                Some((i, '/')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((
                                i,
                                Tok::DivEq(TokLoc {
                                    begin: i,
                                    len: 2,
                                    line: self.line,
                                    char_begin: i - self.line_begin + 1,
                                }),
                                i + 2,
                            )))
                        }
                        _ => Some(Ok((
                            i,
                            Tok::Div(TokLoc {
                                begin: i,
                                len: 1,
                                line: self.line,
                                char_begin: i - self.line_begin + 1,
                            }),
                            i + 1,
                        ))),
                    }
                }
                Some((i, '%')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((
                                i,
                                Tok::ModEq(TokLoc {
                                    begin: i,
                                    len: 2,
                                    line: self.line,
                                    char_begin: i - self.line_begin + 1,
                                }),
                                i + 2,
                            )))
                        }
                        _ => Some(Ok((
                            i,
                            Tok::Mod(TokLoc {
                                begin: i,
                                len: 1,
                                line: self.line,
                                char_begin: i - self.line_begin + 1,
                            }),
                            i + 1,
                        ))),
                    }
                }
                Some((i, '=')) => {
                    return Some(Ok((
                        i,
                        Tok::Eq(TokLoc {
                            begin: i,
                            len: 1,
                            line: self.line,
                            char_begin: i - self.line_begin + 1,
                        }),
                        i + 1,
                    )))
                }
                Some((i, chr)) if chr.is_ascii_alphabetic() || chr == '_' => {
                    return Some(self.parse_identifier(i, chr))
                }
                Some((i, chr)) if chr.is_ascii_digit() => return Some(self.parse_number(i, chr)),

                None => return None, // End of file
                Some((i, _)) => return Some(Err(LexicalError::UnrecognizedToken { location: i })),
            }
        }
    }
}
