fn main() {
    println!("Variables, Constants, Shadowing - Rust\n");

    // implicit declaration (type is missing, rather inferred)
    let x = 4;
    println!("x (implicitly decl.) is: {}", x);
    // explicit declaration
    let x2: u8 = 4;
    println!("x2 (explicitly decl.) is: {}", x2);

    // variables are immutable by default (unless specified otherwise)
    let mut a: u8 = 7;
    println!("\na (before): {}", a);
    a = 9;
    println!("a (after): {}", a);
}