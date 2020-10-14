pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    #[allow(dead_code)]
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 - 100, got {}.", value);
        }

        Guess { value }
    }
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

    #[test]
    #[should_panic]
    fn greater_than_100() {
        let guess = Guess::new(200);
        assert!(guess.value)
    }
}
