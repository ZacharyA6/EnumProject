/******************************************************************************

Rust_4_IterationAndIntrospection

INSTRUCTIONS: 
1. Put original enum Color and main function into comments
2. uncomment the "Beyond Normal Usage" code
3. run the program below it to display beyond normal usage
    - results, langage does not allow a discriminant value assigned more than once


This program shows that Rust does not have built-in enum introspection in the standard library,
so iterating over all enum values requires a manual workaround or an external crate. 
It also demonstrates that Rust rejects duplicate integer discriminants at compile time, 
preventing ambiguous enum values.
*******************************************************************************/

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

const ALL_COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

fn main() {
    println!("Iterating over all enum values in Rust (manual simulation):");

    for color in ALL_COLORS {
        println!("{:?}", color);
    }
}

// NOTE: you can use the strum crate library to iterate through, but manual iteration is more intuitive

/*

// BEYOND NORMAL USAGE CODE BELOW

// example of duplicate values throwing an error, no ambiguity allowed in enums in Rust
enum Status {
    Ok = 1,
    Success = 1, // duplicate discriminant
    Error = 2,
}

fn main() {
    println!("This program will not compile.");
}
*/