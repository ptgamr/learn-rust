use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let result2 = longest_with_an_announcement("xxx", "xxxxx", "Great job!");
    println!("The longest string is {}", result2);

    // ======================

    let novel = String::from("Call me Ishmael. Some years ago...");
    let parts = novel.split('.').next();
    println!("splitted {:#?}", parts);
    let first_sentence = novel.split('.').next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("ImportantExcerpt {}", i.part);
}
