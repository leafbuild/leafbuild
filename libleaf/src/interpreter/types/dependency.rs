pub(crate) trait Dependency {
    fn get_cc_compilation_flags(&self) -> &CCFlags;
    fn get_cc_link_flags(&self) -> &CCLDFlags;

    fn get_cxx_compilation_flags(&self) -> &CXXFlags;
    fn get_cxx_link_flags(&self) -> &CXXLDFlags;
}
