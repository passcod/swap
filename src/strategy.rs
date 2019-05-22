use std::path::Path;

mod renameat2;

#[derive(Debug)]
pub enum Strategy {
    RenameAt2,
}

impl Strategy {
    pub fn swap<F, T>(&self, from: F, to: T) -> Result<(), Error>
    where
        F: AsRef<Path>,
        T: AsRef<Path>,
    {
        match self {
            Strategy::RenameAt2 => renameat2::swap(from, to),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Unimplemented,
    Io(std::io::Error),
    FfiNul(std::ffi::NulError),
}

impl Error {
    pub fn from_last_io() -> Self {
        Error::Io(std::io::Error::last_os_error())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(e: std::ffi::NulError) -> Self {
        Error::FfiNul(e)
    }
}
