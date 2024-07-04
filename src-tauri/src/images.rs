use std::path::{Path};
use std::io::{self, ErrorKind};
use std::fs;

pub fn visit_dirs(dir: &Path) -> io::Result<Vec<String>> {
    if dir.is_dir() {
        let mut files = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut sub_files = visit_dirs(&path)?;
                files.append(&mut sub_files);
            } else {
                files.push(fs::canonicalize(path).unwrap().to_string_lossy().into_owned());
            }
        }
        Ok(files)
    } else {
        Err(io::Error::new(
            ErrorKind::InvalidInput,
            "Given path is not a directory",
        ))
    }
}