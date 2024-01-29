//use std::io::stdin;
#[derive(Debug)]
struct Coffe
{
    id: i32,
    name: String
}
fn main()
{
    //This is how to assign a integer value in rust
    let integer_value: i32 = 10;
    println!("Integer Value: {}", integer_value);

    // Testing out to use to string like in C#
    let parsed_integer: i32 = "123".parse().unwrap();
    println!("Parsed to integer value: {}", parsed_integer);
    println!("Integer to string: {}", parsed_integer.to_string());

    //This is how you assign a floating number in rust
    let float_value: f32 = 10.5;
    println!("Floor: {}", float_value.floor());
    println!("Ceiling: {}", float_value.ceil());
    println!("Rounded: {}", float_value.round()); //This i wonder if there is options for this one.

    let char: char = 'A';

    println!("Is uppercase: {}", char.is_uppercase());
    println!("Is lowercase: {}", char.is_lowercase());
    println!("Lowercase: {}", char.to_ascii_lowercase());
    println!("String version: {}", char.to_string());

    //Sweet tuple looks like fun!
    let tuple: (String, i32, f64) = (String::from("test"), 5, 10.5);

    //To be able to change this tuple after init, you need to say it has to be mutable, right now it is immutable.
    //like this: let mut tuple: (String, i32, f64) = (String::from("test"), 5, 10.5);
    //it is the same with variables, for safety its better to actually use it as immutable.
    ////tuple.0 = String::from("test lite efteråt");
    println!("string, Integer, Float: {}/{}/{}", tuple.0, tuple.1, tuple.2);

    let (test1, test2 ,test3) = tuple;
    println!("test: {} - {} - {}", test1, test2, test3);


    let my_coffe = Coffe{
        id: 10,
        name: String::from("Test String")
    };

    println!("coffe: {:?}", my_coffe);

    //field init shorthand...
    let id = 10;
    let coffe_with_different_name = Coffe{
        id, //<-----------------------This is a shorthand field init, this means it will fill it with 10.
        name: String::from("Hey mY Coffee")
    };

    let combined = Coffe{
        id: 100,
        ..coffe_with_different_name
    };
    println!("Combined: {:?}", combined); //See the name got the name from the other struct
}