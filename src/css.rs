use std::{
    error::Error,
    fs,
    io::BufWriter,
    path::{Path, PathBuf},
    time::Instant,
};

use crate::OUTPUT_DIR;

/// Combine all CSS into a single file
pub fn run() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    let output = PathBuf::from(OUTPUT_DIR).join("bundle.css");
    let output = std::fs::File::create(output)?;
    let mut output = BufWriter::new(output);

    bundle_recurse("css", &mut output)?;

    eprintln!("CSS bundled in {:?}", now.elapsed());

    Ok(())
}

fn bundle_recurse(
    path: impl AsRef<Path>,
    output: &mut impl std::io::Write,
) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            bundle_recurse(path, output)?;
        } else {
            let mut file = fs::File::open(path)?;
            std::io::copy(&mut file, output)?;
        }
    }

    Ok(())
}
