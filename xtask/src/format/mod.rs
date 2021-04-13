/*
 *   Copyright (c) 2021 Dinu Blanovschi
 *   All rights reserved.
 *   Licensed under the terms of the BSD-3 Clause license, see LICENSE for more.
 */
use crate::workspace_root;
use cmd::CmdResult;
use cmd::{cargo_cmd, cmd_call};
use std::path::Path;

pub fn format() -> CmdResult {
    cargo_cmd!("fmt -- --emit=files")
}

pub fn format_check() -> CmdResult {
    cargo_cmd!("fmt -- --check")
}

pub fn format_file(file: &Path) -> CmdResult<()> {
    cmd_call!(&format!(
        "rustfmt --config-path {}/.rustfmt.toml {}",
        workspace_root().display(),
        file.display()
    ))
}
