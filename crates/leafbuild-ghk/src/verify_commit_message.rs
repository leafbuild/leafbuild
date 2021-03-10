use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
enum Err {
    #[error("no argument given")]
    NoArg,
    #[error("io error: ")]
    IoError(#[from] io::Error),
    #[error("not ascii: `{0}`")]
    NotAscii(String),
}

fn main() -> Result<(), Err> {
    let commit_msg_file = std::env::args().nth(1).ok_or(Err::NoArg)?;
    let commit_msg = std::fs::read_to_string(commit_msg_file)?;

    if !commit_msg.is_ascii() {
        return Err(Err::NotAscii(commit_msg));
    }

    Ok(())
}
