fn main() {
    println!("Data Types - Rust\n");

    /*
        Primitve data types
        - basic or fundamental data types used to declare a variable.
            --> scalar type
            --> compound type
    */

    /*
        Scalar types
    */
    let a: i32 = 2; // signed 32-bit integer
    println!("a: {} - signed 32-bit integer", a);
    let b: i8 = -1; // signed 8-bit integer
    println!("b: {} - signed 8-bit integer", b);
    let c: u8 = 5; // unsigned 8-bit integer
    println!("c: {} - unsigned 8-bit integer", c);
    let d: f32 = 3.14; // 32-bit floating point number
    println!("d: {} - 32-bit floating point number", d);
    let e: f64 = 5.287348192374631; // 64-bit floating point number
    println!("e: {} - 64-bit floating point number", e);
    let f: bool = true; // boolean value
    println!("f: {} - boolean type", f);
    let g: char = '$'; // character type
    println!("g: {} - character type", g);

    /*
        Compound types
    */

    /*
        Tuple
        - fixed length sequence of elements that is immutable.
    */
    let tup = (1, true, 's');
    println!("\ntup1 -->");
    println!("\ttup.0: {}", tup.0);
    println!("\ttup.1: {}", tup.1);
    println!("\ttup.2: {}", tup.2);
    
    let mut tup2: (u8, bool, &str) = (7, true, "munir");
    println!("\ntup2 (before) -->");
    println!("\ttup2.0: {}", tup2.0);
    println!("\ttup2.1: {}", tup2.1);
    println!("\ttup2.2: {}", tup2.2);

    tup2.1 = false;
    tup2.2 = "Sami Munir";
    println!("\ntup2 (after) -->");
    println!("\ttup2.0: {}", tup2.0);
    println!("\ttup2.1: {}", tup2.1);
    println!("\ttup2.2: {}", tup2.2);

    /*
        Array
    */
}
