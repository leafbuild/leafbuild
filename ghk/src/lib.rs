#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/leafbuild/leafbuild/master/leaf_icon.svg",
    html_logo_url = "https://raw.githubusercontent.com/leafbuild/leafbuild/master/leaf_icon.svg"
)]
#![forbid(
    unsafe_code,
    unused_allocation,
    coherence_leak_check,
    confusable_idents,
    trivial_bounds
)]
#![deny(
    missing_docs,
    missing_crate_level_docs,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_imports,
    unused_import_braces,
    deprecated,
    broken_intra_doc_links,
    where_clauses_object_safety,
    order_dependent_trait_objects,
    unconditional_panic,
    unconditional_recursion,
    indirect_structural_match
)]
#![deny(
    clippy::correctness,
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::nursery
)]
#![allow(clippy::module_name_repetitions)]

//! # GHK, git hooks

use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use std::io::{Read, Write};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

///
#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum GitHook {
    // Committing workflow
    /// The pre-commit hook.
    ///
    /// Run before commit
    PreCommit,
    /// The prepare-commit-msg hook.
    ///
    /// Run before the editor with
    /// the commit message is fired up.
    PrepareCommitMessage,
    /// The commit-msg hook
    ///
    /// Run after the user typed the
    /// commit message.
    CommitMessage,
    /// The post-commit hook
    ///
    /// Run after the commit.
    /// Generally used to notify.
    PostCommit,

    // Email patch workflow
    ///
    ApplyPatchMessage,
    ///
    PreApplyPatch,
    ///
    PostApplyPatch,

    // Others
    ///
    PreRebase,
    ///
    PostRewrite,
    ///
    PostCheckout,
    ///
    PostMerge,
    ///
    PrePush,
    ///
    PreAutoGC,
}

///
#[derive(Debug, Copy, Clone)]
pub enum ExecutableFileContents<'a> {
    /// Copies the binary from the given path to the hook.
    CopyRawBinary(&'a Path),
    /// Writes the raw binary in the hook file.
    RawBinary(&'a [u8]),
    /// A script with a shebang line
    #[cfg(unix)]
    Script {
        /// The shebang line contents, without the `#!`
        shebang: &'a str,
        /// The script code
        code: &'a str,
    },
    /// An unix symbolic link from this path
    #[cfg(unix)]
    SymbolicLink(&'a Path),
    /// A hard link from this path
    HardLink(&'a Path),
}

impl<'a> ExecutableFileContents<'a> {
    fn write(&self, file: &Path) -> io::Result<File> {
        let f = match self {
            Self::CopyRawBinary(path) => {
                let mut file = File::create(file)?;
                let mut f = File::open(path)?;
                let mut buf = [0; 8192];
                let mut r;

                while {
                    r = f.read(&mut buf)?;
                    r
                } != 0
                {
                    file.write_all(&buf)?;
                }
                file
            }
            Self::RawBinary(raw_bin) => {
                let mut file = File::create(file)?;
                file.write_all(raw_bin)?;
                file
            }
            #[cfg(unix)]
            Self::Script { shebang, code } => {
                let mut file = File::create(file)?;
                writeln!(file, "#!{}", shebang)?;
                if cfg!(feature = "unindent-scripts") {
                    use unindent::Unindent;
                    file.write_all(code.unindent().as_bytes())?;
                } else {
                    file.write_all(code.as_bytes())?;
                }
                file
            }
            #[cfg(unix)]
            Self::SymbolicLink(source) => {
                std::os::unix::fs::symlink(source, file)?;
                File::open(file)?
            }
            Self::HardLink(source) => {
                std::fs::hard_link(source, file)?;
                File::open(file)?
            }
        };

        Ok(f)
    }
}

fn git_directory() -> PathBuf {
    PathBuf::from(".git")
}

impl GitHook {
    /// Returns the git name of this hook
    #[must_use]
    pub const fn git_name(self) -> &'static str {
        match self {
            Self::PreCommit => "pre-commit",
            Self::PrepareCommitMessage => "prepare-commit-msg",
            Self::CommitMessage => "commit-msg",
            Self::PostCommit => "post-commit",
            Self::ApplyPatchMessage => "applypatch-msg",
            Self::PreApplyPatch => "pre-applypatch",
            Self::PostApplyPatch => "post-applypatch",
            Self::PreRebase => "pre-rebase",
            Self::PostRewrite => "post-rewrite",
            Self::PostCheckout => "post-checkout",
            Self::PostMerge => "post-merge",
            Self::PrePush => "pre-push",
            Self::PreAutoGC => "pre-auto-gc",
        }
    }

    /// Installs the `script` script to this hook.
    ///
    /// # Errors
    ///
    /// Any I/O errors that can occur
    pub fn install(self, script: ExecutableFileContents) -> io::Result<()> {
        let mut hook_path = git_directory();
        hook_path.push("hooks");
        hook_path.push(self.git_name());

        let f = script.write(&hook_path)?;

        let mut p = f.metadata()?.permissions();

        #[cfg(unix)]
        p.set_mode(0o755);

        f.set_permissions(p)?;

        Ok(())
    }
}
