fn main() {
    let mut counter = 0;
    let result: &str = loop {
        if counter == 10 {
            break "I'm all lopped out";
        }

        println!("looping!");
        counter += 1;
    };
    println!("{result}");

    counter = 0;
    while counter != 10 {
        println!("while looping!");
        counter += 1;
    }

    let array = [10, 20, 30, 40, 50];

    for a in array {
        println!("The element is {a}");
    }

    for a in (1..4).rev() {
        println!("The element is {a}");
    }
}
