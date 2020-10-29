fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        // need to give a type here because rust can't figure that out
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);

        // can not use v1_iter here
        // because `sum` takes ownership of the iterator
        // --> We call it "Consume the Iterator"

    }

    #[test]
    fn iterator_map_closure() {
        let v1 = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2,3,4]);
    }

    #[test]
    fn iterator_map_fn() {
        let v1 = vec![1, 2, 3];

        fn plus_one(x: &i32) -> i32 {
            x + 1
        }

        let v2: Vec<_> = v1.iter().map(plus_one).collect();

        assert_eq!(v2, vec![2,3,4]);
    }
}
