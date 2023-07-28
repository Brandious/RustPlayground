fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number = 0;

    let x = !(number != 0);

    let y = if x {
        println!("Number was {number}");
        "It was true"
    } else {
        println!("Number was not {number}");
        "It was false"
    };

    println!("{y}");
}
