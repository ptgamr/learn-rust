use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, P, R>
where
    T: Fn(P) -> R,
    R: Copy,
    P: Hash + Eq,
{
    calculation: T,
    values: HashMap<P, R>,
}

impl<T, P, R> Cacher<T, P, R>
where 
    T: Fn(P) -> R,
    R: Copy,
    P: Hash + Eq + Copy,
{
    fn new(calculation: T) -> Cacher<T, P, R> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: P) -> R {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_sepcified_value = 11;
    let simulated_random_number = 7;

    generate_workout(simulated_user_sepcified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        assert_eq!(v1, 1);

        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_string_and_return_usize() {
        let mut c = Cacher::new(|s: &str| s.len());

        let v1 = c.value("abc");
        assert_eq!(v1, 3);

        let v2 = c.value("cdefg");
        assert_eq!(v2, 5);
    }
}
