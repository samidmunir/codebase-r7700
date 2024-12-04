use std::io;

fn main() {
    println!("\nArithmetic & Type Casting - Rust\n");

    /*
    let x: u8 = 9; // 0 -> 255
    let y: i8 = 10; // -128 -> 127

    let z = x + y;
    println!("z: {}", z);
    */

    let xf: f32 = 3.14;
    let yf: f32 = 1.48592;
    let zf  = xf + yf;
    println!("zf: {}", zf);

    let x = 127_000 as i64;
    let y = 10_i32;
    let z = x / (y as i64);
    println!("\nz: {}", z);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("input: {}", input);
    println!("int_input: {}", int_input + 25);
}