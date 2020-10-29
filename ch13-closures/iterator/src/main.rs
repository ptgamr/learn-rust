fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

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
