fn main() {
    let s = "hello"; // immutable reference to a string literal

    let mut string = String::from("hello"); // mutable reference to a String

    let lit_to_str = s.to_string(); // convert a string literal to a String

    string.push_str(", world!");
    println!("s: {}", s);
    println!("string: {}", string);
    println!("litToStr: {}", lit_to_str);

    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("s1: {}", s1); // error: value borrowed here after move

    println!("s2: {}", s2);
    // takes_ownership(s1); // error: value borrowed here after move
    takes_ownership(s2);

    let s3 = gives_ownership();
    println!("s3: {}", s3);

    let s3 = takes_and_gives_back(s3);
    println!("s3: {}", s3);
    
    // println!("s2: {}", s2); // error: value borrowed here after move
}

fn takes_ownership(some_string: String) {
    println!("Borrowed {}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
