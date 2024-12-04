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

    // Rust allows for reuse of variable names if newly declared.
    let z: u8 = 24;
    println!("\nz (old): {}", z);
    let z: u8 = 48;
    println!("z (new): {}", z);

    // name-shadowing
    let b: u8 = 1;
    println!("\n(1) b is: {}", b);

    {
        let b: u8 = 5;
        println!("(2) b is: {}", b);

        let b: u8 = b - b;
        println!("(2a) b is: {}", b);
    }

    let b: u8 = b + 1;
    println!("(3) b is: {}", b);

    let dyn_var: u8 = 7;
    println!("\ndyn_var (before): {}", dyn_var);
    let dyn_var: &str = "Sami";
    println!("dyn_var (after): {}", dyn_var);

    // constants
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("\nSECONDS_IN_MINUTE: {}", SECONDS_IN_MINUTE);

}