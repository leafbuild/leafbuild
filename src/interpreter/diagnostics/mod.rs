use codespan_reporting::files::{Files, SimpleFile};
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Copy, Clone)]
pub struct FileId(usize);

impl FileId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl PartialEq for FileId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for FileId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }

    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
}

pub type LeafbuildFile = SimpleFile<String, String>;

pub struct LeafbuildFiles {
    files: Vec<LeafbuildFile>,
}

impl<'a> LeafbuildFiles {
    pub(crate) fn add(&'a mut self, name: String, source: String) -> FileId {
        self.files.push(LeafbuildFile::new(name, source));
        FileId::new(self.files.len() - 1)
    }
}

impl Default for LeafbuildFiles {
    fn default() -> Self {
        Self { files: vec![] }
    }
}

impl<'a> Files<'a> for LeafbuildFiles {
    type FileId = FileId;
    type Name = &'a String;
    type Source = &'a String;

    fn name(&'a self, id: Self::FileId) -> Option<Self::Name> {
        self.files.get(id.0).map(LeafbuildFile::name)
    }

    fn source(&'a self, id: Self::FileId) -> Option<Self::Source> {
        self.files.get(id.0).map(LeafbuildFile::source)
    }

    fn line_index(&self, file_id: Self::FileId, byte_index: usize) -> Option<usize> {
        self.files.get(file_id.0)?.line_index((), byte_index)
    }

    fn line_range(&self, file_id: Self::FileId, line_index: usize) -> Option<Range<usize>> {
        self.files.get(file_id.0)?.line_range((), line_index)
    }
}
