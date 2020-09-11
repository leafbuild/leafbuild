pub(crate) trait Dependency {
    fn get_compiler_flags(&self, language: Language, env: &Env) -> CompilationFlags;
    fn get_linker_flags(&self, language: Language, env: &Env) -> LinkFlags;
    fn get_implicit_requirements(&self, language: Language, env: &Env) -> Vec<String>;
}
