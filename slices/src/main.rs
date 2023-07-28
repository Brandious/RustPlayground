// fn first_word_initial(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // slice from 0 to i
        }
    }

    return &s[..];
}

fn working_with_slices() {
    let string = String::from("Hello there");

    let hello = first_word(&string[0..6]);
    println!("Hello: {}", hello);
    let hello: &str = first_word(&string[..]);
    println!("Hello: {}", hello);
    let word = first_word(&string);

    println!("The first word is: {}", word);

    let my_string_literal = "Hello there";

    let word = first_word(&my_string_literal[..]);
    println!("The first word is: {}", word);
    let word = first_word(&my_string_literal[0..6]);
    println!("The first word is: {}", word);

    let word = first_word(my_string_literal);
    println!("The first word is: {}", word);
}

fn main() {
    let mut s = String::from("Hello there");

    let word = first_word(&s);

    let hello = &s[0..5];
    let there = &s[6..];

    println!("Hello: {}", hello);
    println!("There: {}", there);
    println!("The first word is: {}", word);

    s.clear();

    working_with_slices();
}
