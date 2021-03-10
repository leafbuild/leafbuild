/// Returns the path to the `ar` binary
/// # Errors
/// If it cannot be found
pub fn get_ar() -> Result<PathBuf, WhichError> {
    which::which("ar")
}
