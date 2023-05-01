fn main() {
    println!("Hello, world!");
    test_one();
    let result = add_numbers(20, 30);

    // expression:
    let number = {
        let x = 3;
        x + 1 // no semicolon to specify "returning"
    }
    // number = 4

}

// Convention for fnctn name: snake case like python
fn test_one() {
    println!("Test!");
}

// Types are mandatory to specify
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y // or return x + y;
}

fn looping() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

    while count == 0 {

    }

    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() { // Use iter() to yield &T, iter_mut() for &mut T, and
        // into_iter() to yield T, &T or &mut T depending on context
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // In Tut4, add classes, dicts, sets, queues, priority queues, heaps, structs etc
}

fn matching() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }
}