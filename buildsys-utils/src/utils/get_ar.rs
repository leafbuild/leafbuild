pub fn get_ar() -> Result<PathBuf, Error> {
    which::which("ar")
}
