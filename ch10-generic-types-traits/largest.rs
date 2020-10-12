fn main() {
    let number_list: Vec<i32> = vec![34, 60, 25, 199, 200, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    println!("The list {:?}", number_list);
}

fn largest(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
