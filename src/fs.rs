use std::fs::{metadata, FileType, Metadata};
use std::path::PathBuf;

pub struct File {
    md: Metadata,
}

impl File {
    pub fn from_path(path: &PathBuf) -> File {
        let md = metadata(path).unwrap();
        File { md }
    }

    #[allow(dead_code)]
    pub fn file_type(&self) -> FileType {
        self.md.file_type()
    }

    pub fn is_file(&self) -> bool {
        self.md.is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.md.is_dir()
    }

    pub fn is_symlink(&self) -> bool {
        self.md.file_type().is_symlink()
    }
}

#[cfg(test)]
mod test {
    use crate::fs::File;
    use std::path::PathBuf;

    #[test]
    fn file_type() {
        let f = File::from_path(&PathBuf::from("./src/commands"));
        assert_eq!(f.file_type().is_file(), false)
    }

    #[test]
    fn is_dir() {
        let f = File::from_path(&PathBuf::from("./src/commands"));
        assert_eq!(f.is_dir(), true)
    }

    #[test]
    fn is_file() {
        let f = File::from_path(&PathBuf::from("./src/fs.rs"));
        assert_eq!(f.is_file(), true)
    }
}
