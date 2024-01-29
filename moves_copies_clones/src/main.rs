#[derive(Debug, Clone)]
struct Coffe {
    id: i32,
    count: i32
}

fn main() {
    let a = 1;
    let b = a;

    println!("a: {}", a);
    println!("b: {}", b);

    let string_a = String::from("Testar min applikation");
    let string_b = string_a;

    //This will cause error due to fact we cant move
    //println!("string_a: {}", string_a);
    //println!("string_b: {}", string_b);

    //But if we do string.clone then it should work..
    let string_g = String::from("Test min ultimata app");
    let string_h = string_g.clone();
    println!("string_g: {}", string_g);
    println!("string_h: {}", string_h);

    {
        let hello_world = String::from("Hello World");
        println!("This hello world only been used within the scope {}", hello_world);
    }

    let coffe_a = Coffe{id: 1, count: 10 };
    let coffe_b = coffe_a;

    //println!("coffe_a: {:?}", coffe_a);
    println!("coffe_b: {:?}", coffe_b);

    let coffe_c = Coffe { id:2, count: 100 };
    let coffe_d = coffe_c.clone();

    println!("coffe_c: {:?}", coffe_c);
    println!("coffe_d: {:?}", coffe_d);

    mutable(&mut String::from("test mutable variable"));
    immutable(String::from("test immutable variable"));
}

fn mutable(dummy_variable: &mut String){
    dummy_variable.push_str(" Some random text"); //this will just append text to the variable pretty neat.
    //i thought this would have worked:
    //to make it works you need to work different with with it, you need to set mut on in parameters oh well, but i guess
    //it would be completely different then, anyway this one works with .push_str
    //dummy_variable = String::from("This also some random text");
    println!("dummy_variable: {}", dummy_variable);
}
fn immutable(dummy_variable: String){
    //This will cause an error.
    //dummy_variable.push_str("Some random text, not very random...");
    println!("dummy_variable: {}", dummy_variable);
}

