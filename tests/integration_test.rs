extern crate blog;

use blog::compile;

#[test]
fn test_compile() {
    if let Err(e) = compile("a", "b") {
        panic!("compile() returned an error: {}", e);
    }
}
