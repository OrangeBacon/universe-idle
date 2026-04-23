//! This is a static site builder for this game, mainly focusing on being a simple
//! javascript bundler.

mod js;
mod root;

use std::{error::Error, fs, time::Instant};

pub const OUTPUT_DIR: &str = "deploy";

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    remove_old()?;

    // create the new output directory, if already exists ignore the error
    _ = fs::create_dir(OUTPUT_DIR);

    // do build work here :)
    root::run()?;
    js::run()?;

    eprintln!("Finished in {:?}", now.elapsed());

    Ok(())
}

/// remove the output directory to clear the previous build files, however keep
/// the `.git` file from git worktree
fn remove_old() -> Result<(), Box<dyn Error>> {
    let Ok(dir) = fs::read_dir(OUTPUT_DIR) else {
        // the output dir doesn't exist, no issue
        return Ok(());
    };

    for item in dir {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else if !path.ends_with(".git") {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
