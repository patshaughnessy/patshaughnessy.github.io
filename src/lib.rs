use std::fs;
use std::path::PathBuf;

pub fn compile(input_path: &PathBuf, _output_path: &PathBuf) -> Result<String, std::io::Error> {
    fs::read_to_string(input_path)
}
