//! Rust app template with DevOps best practices
//!

/// Greet given name
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(String::from("Hello, World!"), greet(String::from("World")))
    }
}
