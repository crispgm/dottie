use std::error::Error;
use std::fmt;

// commands errors
#[derive(Debug, Clone)]
pub struct SymlinkSourceNotSupported;

impl fmt::Display for SymlinkSourceNotSupported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source file cannot be a symbolic link")
    }
}

impl Error for SymlinkSourceNotSupported {}

#[derive(Debug, Clone)]
pub struct SourceFileIsDottied;

impl fmt::Display for SourceFileIsDottied {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source file has already been dottied")
    }
}

impl Error for SourceFileIsDottied {}

#[derive(Debug, Clone)]
pub struct UnknownFileType;

impl fmt::Display for UnknownFileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown file type")
    }
}

impl Error for UnknownFileType {}
