// https://www.youtube.com/watch?v=T_KrYLW4jw8&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=1
// Commands to run in tutorial1 folder:
// cargo fmt (to format)
// cargo build (to build)
// cargo run (to build and run)

fn main() {
    println!("Hello, world!");

    let x = 4;
    // can also do x: u32 = 4;
    println!("x is: {}", x);

    // You cannot now do:
    // x = "hi";
    // because x has a fixed type
    // also because x by default is immutable

    let mut x2 = 4;
    // x is mutable now
    println!("x2 is: {}", x2);

    // This is allowed: (redeclaration)
    let x = 5;
    let x = x + 1;
    let x = "hello";

    // Scoping
    {
        let x = 2;
    }

    // Constants (must specify type)
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);

    // Integer: i8, i16, i32, i64, i128
    // also u8, u16... (unsigned)
    // Default for integer is i32 if type is not specified.
    let x: i32 = 2;

    // Floats, f32 and f64 (default is f64)
    let fl: f32 = 10.9;

    // Bool
    let b: bool = true;

    // Char
    let letter: char = 'a';

    // Tuple
    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.0);

    let mut tup: (i32, bool, char) = (1, true, 's');
    tup.0 = 2;

    // Arrays
    let mut arr = [1, 2, 3, 4, 5];
    // reference by arr[0]
    arr[4] = 6;
    println!("{}", arr[4]);
    let mut arr: [i32; 5]; // Uninitialised values inside!

    

}
