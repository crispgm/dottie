use std::fs::{metadata, FileType, Metadata};
use std::path::PathBuf;

pub struct File {
    path: PathBuf,
    md: Metadata,
}

impl File {
    pub fn from_path(path: PathBuf) -> File {
        let p = PathBuf::from(path);
        let md = metadata(p.clone()).unwrap();
        File { path: p, md }
    }

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

    #[cfg(test)]
    fn is_dir() {
        let f = File::from_path(PathBuf::from("./commands"));
        assert_eq!(f.is_dir(), true)
    }

    #[cfg(test)]
    fn is_file() {
        let f = File::from_path(PathBuf::from("./fs.rs"));
        assert_eq!(f.is_file(), true)
    }
}
