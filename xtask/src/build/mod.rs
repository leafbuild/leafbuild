use cmd::cargo_cmd;
use cmd::CmdResult;

pub fn build() -> CmdResult {
    cargo_cmd!("build")
}
