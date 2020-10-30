use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// The Deref trait, provided by standard library
// requires to implement one method named `deref` that borrow `self` and
// returns a reference to the inner data.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // when we enter *y
    // behind the scenes, Rust actually ran this code
    // *(y.deref)
    //
    // Rust substitues the * operator with a call to `deref()` and then a plain dereference
    // so we don't have to think about whether or not we need to call the deref method.
}
