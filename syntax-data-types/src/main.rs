use std::io::stdin;

fn main() {
    //This is how to assign a integer value in rust
    let integerValue: i32 = 10;
    println!("Intenger Value: {}", integerValue);

    // Testing out to use to string like in C#
    let parsedInteger: i32 = "123".parse().unwrap();
    println!("Parsed to integer value: {}", parsedInteger);
    println!("Integer to string: {}", parsedInteger.to_string());

    let floatValue: f32 = 10.5;
    println!("Floor: {}", floatValue.floor());
    println!("Ceiling: {}", floatValue.ceil());
    println!("Rounded: {}", floatValue.round()); //This i wonder if there is options for this one.
}