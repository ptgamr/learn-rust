pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    #[allow(dead_code)]
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1");
        }

        if value > 1 {
            panic!("Guess value must be less than or equal to 100");
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
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
