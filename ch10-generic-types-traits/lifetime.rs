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

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);

    // ======================

    let first_sentence;

    {
        let novel;
        novel = String::from("Call me Ishmael. Some years ago...");
        let parts = novel.split('.').next();
        println!("splitted {:#?}", parts);
        first_sentence = novel.split('.').next().unwrap().clone();
    }

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("ImportantExcerpt {}", i.part);
}
