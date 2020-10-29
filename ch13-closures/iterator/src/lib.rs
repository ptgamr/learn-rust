#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_my_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<&Shoe> {
    // have to use into_iter
    // to create an iterator that takes ownership of the vector.... 
    //
    // BUT WHY do we need to take the ownership????
    // 
    // filter will adapt that iterator into a new iterator that contains only elements
    // for which the closure returns `true`
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// ===================
// CUSTOM ITERATOR
// A simpler that returning the values from 1 --> 5
// ===================
struct Counter {
    count: u32,
}

impl Counter {
    #[allow(dead_code)]
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(&shoes, 10);


        println!("Shoes {:?}", shoes);

        assert_eq!(
            in_my_size,
            vec![
                &shoes[0],
                &shoes[2],
            ]
        );
    }

    #[test]
    fn calling_custom_iterator() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_method() {
        // all of these method calls are possible because we specified how the `next` method
        // works, and the standard library provides default implementations for other method
        // that call `next`
        let sum: u32 = Counter::new()
            // zip produces only for pair
            // because the theoretical fifth pair (5, None) is never produced
            // zip returns None when either of its input iterators return None
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);
    }
}
