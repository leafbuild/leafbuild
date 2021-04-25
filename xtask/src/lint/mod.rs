use cmd::cargo_cmd;
use cmd::CmdResult;

pub fn lint() -> CmdResult {
    cargo_cmd!("clippy")
}
