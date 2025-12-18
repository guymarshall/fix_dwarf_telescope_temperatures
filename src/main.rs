use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry: fs::DirEntry = entry?;
            let path: std::path::PathBuf = entry.path();

            if path.is_dir() {
                visit_dirs(&path)?;
            } else if is_fits_file(&path) {
                process_file(&path)?;
            }
        }
    }
    Ok(())
}

fn is_fits_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e: &std::ffi::OsStr| e.to_str())
        .map(|e: &str| e.eq_ignore_ascii_case("fits"))
        .unwrap_or(false)
}

fn process_file(path: &Path) -> io::Result<()> {
    let file: std::borrow::Cow<'_, str> = path.to_string_lossy();

    let status: std::process::ExitStatus = Command::new("grep")
        .arg("-q")
        .arg("CCD-TEMP")
        .arg(&*file)
        .status()?;

    if status.success() {
        return Ok(());
    }

    let status: std::process::ExitStatus = Command::new("perl")
        .arg("-pi")
        .arg("-e")
        .arg("s/DET-TEMP/CCD-TEMP/g")
        .arg(&*file)
        .status()?;

    println!("Processed {file}");

    if !status.success() {
        eprintln!("  Error processing {}", file);
    }

    Ok(())
}

// TODO: Replace Perl with Rust
// TODO: Parallelise

fn main() -> io::Result<()> {
    let root: &Path = Path::new(".");

    visit_dirs(root)?;

    Ok(())
}
