use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn compile(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(input_path)?;
    let mut file = File::create(&output_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
