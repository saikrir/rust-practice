use std::{
    fs,
    path::{self, Path},
};

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        if let Ok(_) = fs::read(&self) {
            return true;
        }
        return false;
    }

    fn is_writeable(&self) -> bool {
        if let Ok(m) = fs::metadata(&self) {
            return !m.permissions().readonly();
        }
        return false;
    }

    fn exists(&self) -> bool {
        if let Ok(exists) = fs::exists(self) {
            return exists;
        }
        return false;
    }
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}
