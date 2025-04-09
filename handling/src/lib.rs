use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create(file: &Path, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file)
        .expect("Failed to open or create file");

    writeln!(file, "{}", content).unwrap();
}