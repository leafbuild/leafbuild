use crate::cargo_cmd;
use crate::cmd::CmdResult;

pub fn format() -> CmdResult {
    cargo_cmd!("fmt -- --emit=files")
}

pub fn format_check() -> CmdResult {
    cargo_cmd!("fmt -- --check")
}
