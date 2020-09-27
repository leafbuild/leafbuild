use crate::interpreter::types::{Executable, Library};

pub(crate) mod ninja;

pub(in crate::interpreter) trait GenForLeafbuild {
    fn gen_exe(&mut self, exe: &Executable);
    fn gen_lib(&mut self, lib: &Library);

    // fn gen_compile_source(&mut self, source: &Source);
}
