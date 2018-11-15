pub fn some_string() -> String {
    "some-string-value".to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_some_string() {
        assert_eq!("some-string-value", some_string());
    }
}
