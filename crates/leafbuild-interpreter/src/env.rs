//! The environment of the interpreter.
use crate::LfModName;
use leafbuild_core::diagnostics::FileId;
use std::marker::PhantomData;

/// A file frame, used to hold all the context information of a single file during execution,
/// For example names and values of variables and constants, declared types, functions, ....
#[derive(Debug)]
pub struct FileFrame<'frame> {
    file_id: FileId,
    mod_name: LfModName,
    __phantom: ::std::marker::PhantomData<&'frame ()>,
}

impl<'frame> FileFrame<'frame> {
    pub(crate) const fn new(file_id: FileId, mod_name: LfModName) -> Self {
        Self {
            file_id,
            mod_name,
            __phantom: PhantomData,
        }
    }
}
