use std::{
    error::Error,
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use crate::OUTPUT_DIR;

/// Process typescript and bundle it to a single js file
pub fn run() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    eprintln!("Running tsc...");

    let status = Command::new("npx").arg("tsc").status()?;
    if !status.success() {
        return Err("tsc failed".into());
    }
    eprintln!("Took {:?}", now.elapsed());

    let now = Instant::now();
    bundle(BUNDLE_ROOT)?;
    eprintln!("JS bundled in {:?}", now.elapsed());

    Ok(())
}

const BUNDLE_ROOT: &str = "target/ts";
const BUNDLE_TEMPLATE: &str = include_str!("bundle_template.js");

/// custom javascript bundler oop, designed to do as little as possible
fn bundle(path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let output = PathBuf::from(OUTPUT_DIR).join("bundle.js");
    let output = std::fs::File::create(output)?;
    let mut output = BufWriter::new(output);

    let (before, after) = BUNDLE_TEMPLATE.split_once("...").unwrap();

    writeln!(output, "{before}")?;
    bundle_recurse(path, &mut output)?;
    writeln!(output, "{after}")?;

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
            bundle_write(path, output)?;
        }
    }

    Ok(())
}

fn bundle_write(
    path: impl AsRef<Path>,
    output: &mut impl std::io::Write,
) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    let require_path = path.strip_prefix(BUNDLE_ROOT)?.with_extension("");
    let mut file = fs::File::open(path)?;

    write!(output, "\"{}\": (exports) => {{", require_path.display())?;
    std::io::copy(&mut file, output)?;
    writeln!(output, "}},")?;

    Ok(())
}
