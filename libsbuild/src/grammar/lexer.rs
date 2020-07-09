pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Clone, Debug)]
pub enum Tok {
    Newline,
    Number(i32),
    Identifier(String),

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    Eq,

    POPEN,
    PCLOSE,
    Colon,
    Comma,
    Dot,
}

#[derive(Debug)]
pub enum LexicalError {
    UnrecognizedToken { location: usize }
}

use std::str::CharIndices;
use itertools::Itertools;
use std::borrow::Borrow;
use std::iter::Peekable;

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

    fn parse_identifier(&mut self, initial_position: usize, initial_letter: char) -> Result<(usize, Tok, usize), LexicalError> {
        let result: String;
        let mut next_position: usize = initial_position + 1;
        result = format!("{}{}", initial_letter,
                         self.chars.peeking_take_while(|(pos, chr)| -> bool {
                             next_position = *pos;
                             chr.is_ascii_alphanumeric()
                         }).map(|(_pos, chr)| chr).collect::<String>());
        Ok((initial_position, Tok::Identifier(result), next_position))
    }

    fn parse_number(&mut self, initial_position: usize, initial_char: char) -> Result<(usize, Tok, usize), LexicalError> {
        match self.chars.peek() {
            None => Ok((initial_position, Tok::Number(Self::decdigit_value(initial_char)), initial_position + 1)),
            Some((i, chr)) => {
                if initial_char == '0' {
                    if *chr == 'x' {
                        // parse as hex
                        self.chars.next().unwrap(); // take the 'x' out of the stream
                        let mut num = 0;
                        let end_position;
                        loop {
                            match self.chars.peek() {
                                Some((pos, character))  if character.is_ascii_hexdigit() => {
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
                        Ok((initial_position, Tok::Number(num), end_position))
                    } else {
                        // parse as oct
                        let mut num = 0;
                        let end_position;
                        loop {
                            match self.chars.peek() {
                                Some((pos, character))  if character.is_digit(8) => {
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
                        Ok((initial_position, Tok::Number(num), end_position))
                    }
                } else {
                    let mut num = 0;
                    let end_position;
                    loop {
                        match self.chars.peek() {
                            Some((pos, character))  if character.is_ascii_digit() => {
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
                    Ok((initial_position, Tok::Number(num), end_position))
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
            _ => 0
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
                Some((i, ':')) => return Some(Ok((i, Tok::Colon, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Tok::Comma, i + 1))),
                Some((i, '.')) => return Some(Ok((i, Tok::Dot, i + 1))),
                Some((i, '(')) => return Some(Ok((i, Tok::POPEN, i + 1))),
                Some((i, ')')) => return Some(Ok((i, Tok::PCLOSE, i + 1))),
                Some((i, '+')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Tok::AddEq, i + 2)))
                    }
                    _ => Some(Ok((i, Tok::Add, i + 1)))
                },
                Some((i, '-')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Tok::SubEq, i + 2)))
                    }
                    _ => Some(Ok((i, Tok::Sub, i + 1)))
                },
                Some((i, '*')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Tok::MulEq, i + 2)))
                    }
                    _ => Some(Ok((i, Tok::Mul, i + 1)))
                },
                Some((i, '/')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Tok::DivEq, i + 2)))
                    }
                    _ => Some(Ok((i, Tok::Div, i + 1)))
                },
                Some((i, '%')) => return match self.chars.peek() {
                    Some((_, '=')) => {
                        self.chars.next();
                        Some(Ok((i, Tok::ModEq, i + 2)))
                    }
                    _ => Some(Ok((i, Tok::Mod, i + 1)))
                },
                Some((i, '=')) => return Some(Ok((i, Tok::Eq, i + 1))),
                Some((i, chr)) if chr.is_ascii_alphabetic() => return Some(self.parse_identifier(i, chr)),
                Some((i, chr)) if chr.is_ascii_digit() => return Some(self.parse_number(i, chr)),

                None => return None, // End of file
                Some((i, _)) => return Some(Err(LexicalError::UnrecognizedToken { location: i })),
            }
        }
    }
}