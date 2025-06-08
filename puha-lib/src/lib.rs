/// Returns a greeting string from `puha-lib`.
pub fn greet() -> &'static str {
    "Hello from puha-lib!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_expected() {
        assert_eq!(greet(), "Hello from puha-lib!");
    }
}
