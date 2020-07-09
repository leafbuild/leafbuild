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

    POPEN,
    PCLOSE,
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
        let result: String;
        let mut next_position: usize = initial_position + 1;
        let type_ = if char == '0' { 0 } else { 1 };
        match self.chars.peek() {
            None => Ok((initial_position, Tok::Number(initial_char - '0'), initial_position + 1)),
            Some((i, chr)) => {
                if initial_char == '0' {
                    if chr == 'x' {
                        // parse as hex
                        self.chars.next().unwrap(); // take the 'x' out of the stream
                        let (mut num, mut end_position) = (0, initial_position + 1);
                        while let Some((pos, character)) = self.chars.next() {
                            end_position = pos;
                            if character.is_ascii_hexdigit() {
                                num = num * 16 + hexdigit_value(character);
                            } else {
                                break;
                            }
                        }
                        Ok((initial_position, Tok::Number(num), end_position))
                    } else {
                        // parse as oct
                        let (mut num, mut end_position) = (0, initial_position + 1);
                        while let Some((pos, character)) = self.chars.next() {
                            end_position = pos;
                            if character.is_digit(8) {
                                num = num * 8 + octdigit_value(character);
                            } else {
                                break;
                            }
                        }
                        Ok((initial_position, Tok::Number(num), end_position))
                    }
                }
            }
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, '\n')) => return Some(Ok((i, Tok::Newline, i + 1))),
                Some((_, chr)) if chr.is_whitespace() => continue,
                Some((i, chr)) if chr.is_ascii_alphabetic() => return Some(self.parse_identifier(i, chr)),
                Some((i, chr)) if chr.is_ascii_digit() => return Some(self.parse_number(i, chr)),

                None => return None, // End of file
                Some((i, _)) => return Some(Err(LexicalError::UnrecognizedToken { location: i })),
            }
        }
    }
}