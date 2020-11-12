pub mod constants;

#[must_use]
pub fn repo_file(name: &str) -> String {
    format!(
        "{}/blob/{}/{}",
        constants::REPO,
        constants::REPO_DEFAULT_BRANCH,
        name
    )
}

#[must_use]
pub fn pr_link(id: impl Into<u32>) -> String {
    format!("{}/{}", constants::PR_ROOT, id.into())
}

#[must_use]
pub fn issue_link(id: impl Into<u32>) -> String {
    format!("{}/{}", constants::ISSUE_TRACKER, id.into())
}
