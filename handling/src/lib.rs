use std::fs::OpenOptions;
use std::io::Write;

fn open_or_create(file: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file)
        .expect("Failed to open or create file");

    writeln!(file, "{}", content).unwrap();
}