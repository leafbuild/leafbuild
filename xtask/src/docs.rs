use cmd::{cmd_call, CmdResult};

use crate::ws_path;

pub fn build() -> CmdResult {
    let p = ws_path!("doc");
    cmd_call!("mdbook build", workdir = p)
}

pub fn serve() -> CmdResult {
    let p = ws_path!("doc");
    cmd_call!("mdbook serve", workdir = p)
}
