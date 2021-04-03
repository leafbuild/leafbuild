use crate::cargo_cmd;
use crate::cmd::CmdResult;

pub fn build() -> CmdResult {
    cargo_cmd!("build")
}
