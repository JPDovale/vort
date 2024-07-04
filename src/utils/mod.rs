use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn write_file(content: &str) -> Result<(), std::io::Error> {
    let build_dir = "test/build";
    let path = format!("{}/test.rs", build_dir);

    if !Path::new(build_dir).is_dir() {
        fs::create_dir(build_dir)?;
    }

    if Path::new(path.as_str()).exists() {
        fs::remove_file(path.as_str())?;
    }

    let mut output = File::create(path.as_str())?;
    output.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_file(filename: &str) -> Result<String, io::Error> {
    let file = File::open(filename)?;
    let mut content = String::new();

    for line in BufReader::new(file).lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    Ok(content)
}
