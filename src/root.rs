use std::{error::Error, fs, path::PathBuf};

use crate::OUTPUT_DIR;

/// Copy files from /root into the deploy folder recursively
pub fn run() -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir("root")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            todo!()
        } else if let Some(name) = path.file_name() {
            fs::copy(&path, PathBuf::from(OUTPUT_DIR).join(name))?;
        }
    }

    Ok(())
}
