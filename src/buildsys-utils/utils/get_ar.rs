/// Returns the path buf to the `ar` binary, or an error if it cannot be found
/// # Errors
/// If it cannot be found
pub fn get_ar() -> Result<PathBuf, Error> {
    which::which("ar")
}
