use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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

fn file_contains_ccd_temp(path: &Path) -> io::Result<bool> {
    let mut file: File = File::open(path)?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    Ok(buf
        .windows(b"CCD-TEMP".len())
        .any(|w: &[u8]| w == b"CCD-TEMP"))
}

fn replace_det_with_ccd(path: &Path) -> io::Result<()> {
    let data: Vec<u8> = fs::read(path)?;
    let mut out: Vec<u8> = Vec::with_capacity(data.len());

    let mut i: usize = 0;
    while i < data.len() {
        if i + 8 <= data.len() && &data[i..i + 8] == b"DET-TEMP" {
            out.extend_from_slice(b"CCD-TEMP");
            i += 8;
        } else {
            out.push(data[i]);
            i += 1;
        }
    }

    fs::write(path, out)?;
    Ok(())
}

fn process_file(path: &Path) -> io::Result<()> {
    let file: std::borrow::Cow<'_, str> = path.to_string_lossy();

    if file_contains_ccd_temp(path)? {
        return Ok(());
    }

    replace_det_with_ccd(path)?;

    println!("Processed {file}");

    Ok(())
}

fn main() -> io::Result<()> {
    let root: &Path = Path::new(".");

    visit_dirs(root)?;

    Ok(())
}
