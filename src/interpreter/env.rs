use crate::handle::config::EnvConfig;
use crate::interpreter::diagnostics::{FileId, LeafbuildFiles};
use std::marker::PhantomData;

#[derive(Default)]
pub(crate) struct Env<'env> {
    files: LeafbuildFiles,
    __phantom: ::std::marker::PhantomData<&'env ()>,
}

impl<'env> Env<'env> {
    pub(crate) fn new(_config: EnvConfig) -> Self {
        Self {
            files: LeafbuildFiles::default(),
            __phantom: PhantomData,
        }
    }

    pub(crate) fn register_new_file(&mut self, name: String, source: String) -> FileId {
        self.files.add(name, source)
    }
}

pub(crate) struct EnvFrame<'frame> {
    file_id: FileId,
    __phantom: ::std::marker::PhantomData<&'frame ()>,
}

impl<'frame> EnvFrame<'frame> {
    pub(crate) fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            __phantom: PhantomData,
        }
    }
}
