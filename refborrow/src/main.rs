fn reference_flow() {
    let mut s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;
    println!("{} and {}", r1, r2);

    let r3 = &mut s1;
    println!("{}", r3);
}

fn main() {
    // let s = String::from("Hello there"); // can't be changed

    let mut s = String::from("Hello there"); // can be changed

    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    change(&mut s);

    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    reference_flow();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(s: &String) {
//     s.push_str(", world!"); // doesn't work
// }

fn change(s: &mut String) {
    s.push_str(", world!");
}
