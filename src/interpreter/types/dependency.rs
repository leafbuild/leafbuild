pub(crate) trait Dependency {
    fn get_implicit_requirements(&self, language: Language, env: &Env) -> Vec<String>;
}
