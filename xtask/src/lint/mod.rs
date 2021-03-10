use crate::cargo_cmd;
use crate::cmd::CmdResult;

pub fn lint() -> CmdResult {
    cargo_cmd!("clippy")
}
