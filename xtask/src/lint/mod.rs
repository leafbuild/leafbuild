use crate::cmd::CmdResult;
use crate::cmd_call;

fn lint() -> CmdResult {
    cmd_call!("cargo clippy")
}
