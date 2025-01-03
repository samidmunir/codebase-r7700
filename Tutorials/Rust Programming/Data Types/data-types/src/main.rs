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
        - type is defined by size and kind of element.
        - fixed length/size
        - can use mut keyword for mutability.
    */
    let arr = [1, 2, 3, 4, 5];
    println!("\narr -->");
    println!("\tarr[0]: {}", arr[0]);
    println!("\tarr[1]: {}", arr[1]);
    println!("\tarr[2]: {}", arr[2]);
    println!("\tarr[3]: {}", arr[3]);
    println!("\tarr[4]: {}", arr[4]);

    let arr2: [char; 4] = ['s', 'a', 'm', 'i'];
    println!("\narr2 -->");
    println!("\tarr2[0]: {}", arr2[0]);
    println!("\tarr2[1]: {}", arr2[1]);
    println!("\tarr2[2]: {}", arr2[2]);
    println!("\tarr2[3]: {}", arr2[3]);

    /*
        Strings
    */
}
