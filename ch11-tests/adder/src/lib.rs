pub fn greeting(name: &str) -> String {
    format!("Hello!")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Anh");
        assert!(
            result.contains("Anh"),
            "Greeting did not contain name, value was {}",
            result,
        );
    }
}
