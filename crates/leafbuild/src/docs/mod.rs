//! Module related to documentation. Mostly utility functions.
use std::fmt::Display;

pub mod constants;

/// Creates and returns a link to the source code of a file on the default branch of the repo.
///
/// # Examples
///
/// ```rust
/// use leafbuild::docs::repo_file;
/// assert_eq!(repo_file("src/main.rs"), "https://github.com/leafbuild/leafbuild/blob/master/src/main.rs");
/// assert_eq!(repo_file("src/lib.rs"), "https://github.com/leafbuild/leafbuild/blob/master/src/lib.rs");
/// ```
#[must_use]
pub fn repo_file(name: impl Display) -> String {
    format!(
        "{}/blob/{}/{}",
        constants::REPO,
        constants::REPO_DEFAULT_BRANCH,
        name
    )
}

/// Creates and returns a link to the PR with the given `id`.
///
/// # Examples
///
/// ```rust
/// use leafbuild::docs::pr_link;
///
/// assert_eq!(pr_link(1_u32), "https://github.com/leafbuild/leafbuild/pull/1");
/// assert_eq!(pr_link(2_u32), "https://github.com/leafbuild/leafbuild/pull/2");
/// ```
#[must_use]
pub fn pr_link(id: impl Into<u32>) -> String {
    format!("{}/{}", constants::PR_ROOT, id.into())
}

/// Creates and returns a link to the issue with the given `id`.
///
/// # Examples
///
/// ```rust
/// use leafbuild::docs::issue_link;
///
/// assert_eq!(issue_link(1_u32), "https://github.com/leafbuild/leafbuild/issues/1");
/// ```
#[must_use]
pub fn issue_link(id: impl Into<u32>) -> String {
    format!("{}/{}", constants::ISSUE_TRACKER, id.into())
}
