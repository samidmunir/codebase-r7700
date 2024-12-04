
// standard input/output library
use std::io;

fn main() {
    println!("\nConsole Input - Rust\n");

    /*
        The prelude is the list of things that Rust automatically imports into every Rust program.

        It is kept small as possible, and is focused on things, particularly traits, which are used in almost
         every single Rust program.
    */

    // path-separator operator
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("\nInput enterd: {}", input);
}
