fn main() {
    let x = 5; // immutable variable
    let mut y = 5; // mutable variable

    const SOME_CONSTANT: u32 = 30 * 29 + 1;
    const STRING_CONSTANT: &str = "Hello, world!";

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of SOME_CONSTANT is: {SOME_CONSTANT}");
    println!("The value of STRING_CONSTANT is: {STRING_CONSTANT}");

    let x = x + 1; // shadowing start
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}"); // shadowing end

    y = 9;
    println!("The value of y is: {y}");
}
