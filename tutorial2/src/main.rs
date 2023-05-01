use std::io;

fn main() {
    println!("Hello, world!");

    // use String package
    let mut input = String::new();

    // Get reference using &, references are immutable by default
    // we need the .expect!
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    println!("{}", input);

    // Arithmetic
    let x: u8 = 255;
    let y: u8 = 10;
    let z = x / y; // 25. Can also do x % y

    // Another way to set x's type, and casting
    let x = 127000_i64;
    let y = 127 as i32;
    let z = x / (y as i64);
    println!("{}", z);

    // get max with i32::MAX

    // Parse string
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");
    // .unwrap gives us the actual integer result, parse just does the conversion.
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", input);

    // 2 <= 2 is valid, but 2 <= 2.2 is invalid - types must match
    // conditional operators: && || !
    let food = "cookie";

    if food == "cookie" {
        println!("I like cookies!");
    } else if food == "fruit" {
        println!("Nice");
    } else {
        println!("Ok?");
    }


}
