use std::{convert, fmt, io};

#[derive(Debug)]
enum Err {
    NoArg,
    IoError(io::Error),
    NotAscii(String),
}

impl convert::From<io::Error> for Err {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Err::NoArg => write!(f, "no argument given"),
            Err::IoError(e) => write!(f, "io error: {}", e),
            Err::NotAscii(s) => write!(f, "not ascii: \"{}\"", s),
        }
    }
}

fn main() -> Result<(), Err> {
    let commit_msg_file = std::env::args().nth(1).ok_or(Err::NoArg)?;
    let commit_msg = std::fs::read_to_string(commit_msg_file)?;

    match commit_msg.is_ascii() {
        true => Ok(()),
        false => Err(Err::NotAscii(commit_msg)),
    }
}
