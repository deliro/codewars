fn camel_case(str: &str) -> String {
    str.split_ascii_whitespace()
        .map(|w| w[..1].to_ascii_uppercase() + &*w[1..].to_ascii_lowercase())
        .collect()
}

// Rust tests
#[test]
fn sample_test() {
    assert_eq!(camel_case("test case"), "TestCase");
    assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
    assert_eq!(camel_case("say hello "), "SayHello");
    assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
    assert_eq!(camel_case(""), "");
}
