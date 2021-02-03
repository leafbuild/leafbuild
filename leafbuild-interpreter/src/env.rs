//! The environment of the interpreter.
use crate::handle::Handle;
use crate::internal::values::Value;
use crate::LfModName;
use leafbuild_core::diagnostics::FileId;
use std::collections::HashMap;
use std::marker::PhantomData;

/// A file frame, used to hold all the context information of a single file during execution,
/// For example names and values of variables and constants, declared types, functions, ....
#[derive(Debug)]
pub struct FileFrame<'frame, 'handle>
where
    'handle: 'frame,
{
    pub(crate) file_id: FileId,
    pub(crate) mod_name: LfModName,
    pub(crate) handle: &'frame mut Handle<'handle>,
    pub(crate) __phantom: ::std::marker::PhantomData<&'frame ()>,
}

impl<'frame, 'handle> FileFrame<'frame, 'handle>
where
    'handle: 'frame,
{
    pub(crate) fn new(
        file_id: FileId,
        mod_name: LfModName,
        handle: &'frame mut Handle<'handle>,
    ) -> Self {
        Self {
            file_id,
            mod_name,
            handle,
            __phantom: PhantomData,
        }
    }
}

/// Name lookup data. A stack of those make up a file frame
#[derive(Debug)]
pub struct SemiFrame<'frame> {
    name_lookup: NameLookup<'frame>,

    parent_frame: Option<&'frame SemiFrame<'frame>>,
}

/// A name lookup table
#[derive(Debug)]
pub struct NameLookup<'frame> {
    variables: HashMap<String, Box<dyn Value<'frame>>>,
}

impl<'frame> NameLookup<'frame> {
    /// Returns the value of a variable in this name lookup with the given name
    #[must_use]
    pub fn lookup_variable(&'frame self, name: &str) -> Option<&'frame dyn Value<'frame>> {
        let v = self.variables.get(name);
        v.map(|it| &**it)
    }
    /// Returns the value of a variable in this name lookup with the given name (mutable variant)
    #[must_use]
    pub fn lookup_variable_mut(
        &'frame mut self,
        name: &str,
    ) -> Option<&'frame mut dyn Value<'frame>> {
        let v = self.variables.get_mut(name);
        v.map(|it| -> &'frame mut dyn Value<'frame> { &mut **it })
    }
}
