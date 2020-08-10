//! # Stuff related to the compilers

use std::env::VarError;
use std::io;
use std::path::PathBuf;
use std::string::FromUtf8Error;

// c compilers
pub mod cc;

// c++ compilers
pub mod cxx;

#[derive(Debug)]
pub enum GetCompilerError {
    VarError(VarError),
    ProcessError(io::Error),
    InvalidUtf8(FromUtf8Error),
    UnrecognizedCompilerFamily(String),
}

impl From<VarError> for GetCompilerError {
    #[inline]
    fn from(v: VarError) -> Self {
        GetCompilerError::VarError(v)
    }
}

impl From<io::Error> for GetCompilerError {
    #[inline]
    fn from(v: io::Error) -> Self {
        GetCompilerError::ProcessError(v)
    }
}

impl From<FromUtf8Error> for GetCompilerError {
    #[inline]
    fn from(v: FromUtf8Error) -> Self {
        GetCompilerError::InvalidUtf8(v)
    }
}

pub trait Compiler {
    fn can_consume(filename: &str) -> bool;
    fn can_compile(filename: &str) -> bool;

    fn get_location(&self) -> &PathBuf;
}
