extern crate blog;

use blog::some_string;

#[test]
fn test_one() {
    assert_eq!("some-string-value", some_string());
}
