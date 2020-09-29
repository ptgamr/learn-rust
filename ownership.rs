fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function
                                    // ... and no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move to the function
                                    // but i32 is Copy, so it's ok to 
                                    // still use x afterward

    println!("After {}", x);        // println still works here
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}


fn makes_copy(some_integer: i32) {
    println!("Makes copy {}", some_integer);
}
