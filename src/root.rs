use std::{
    error::Error,
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::OUTPUT_DIR;

/// Copy files from /root into the deploy folder recursively
pub fn run() -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir("root")? {
        recurse(entry?)?;
    }

    Ok(())
}

fn recurse(entry: DirEntry) -> Result<(), Box<dyn Error>> {
    let path = entry.path();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            recurse(entry?)?;
        }
    } else if path.is_file() {
        let mut out = path.as_path();
        if let Ok(p) = out.strip_prefix("root") {
            out = p;
        }
        let out = PathBuf::from(OUTPUT_DIR).join(out);

        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(&path, &out)?;
    }

    Ok(())
}
