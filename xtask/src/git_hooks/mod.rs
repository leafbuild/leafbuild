use std::path::PathBuf;

use crate::err;
use std::io;

pub fn install_git_hooks() -> io::Result<()> {
    use ghk::ExecutableFileContents::*;
    use ghk::GitHook::*;
    #[cfg(unix)]
    PreCommit.install(Script {
            shebang: "/usr/bin/env bash",
            code:
            // language=bash
            r#"
            cargo xtask prepare-commit || exit $?
            "#,
        })?;
    #[cfg(unix)]
    CommitMessage.install(Script {
            shebang: "/usr/bin/env bash",
            code:
            // language=bash
            &r#"
            cargo xtask verify-commit-message $1 || exit $?
            "#,
        })?;

    Ok(())
}

pub fn prepare_commit() {
    // super::test::test();
}

pub fn verify_commit_message(p: PathBuf) {
    let commit_msg = std::fs::read(&p).unwrap_or_else(|err| err!("Cannot open file: {}", err));
    let commit_msg =
        String::from_utf8(commit_msg).unwrap_or_else(|err| err!("Cannot parse utf8: {}", err));
    if !commit_msg.is_ascii() {
        err!("Commit message contains a char that is not ascii");
    }
}
