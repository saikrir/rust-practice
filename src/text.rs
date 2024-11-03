use std::fmt::Display;

pub fn info<T: Display>(text: T) -> () {
    println!("info {}", text);
}

#[test]
fn test_print() {
    let text = "Hello World";
    info(text);
    let text = String::from("Katterishetty");
    info(text);
}
