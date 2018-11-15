use std::error::Error;

pub fn compile(_input_path: &str, _output_path: &str) -> Result<(), Box<Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compile() {
        if let Err(e) = compile("a", "b") {
            panic!("compile() returned an error: {}", e);
        }
    }
}
