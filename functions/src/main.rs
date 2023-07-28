fn main() {
    let y = 7; // this is a statement, doesnt return a value

    another_function(y, "tomatoes");

    let y = {
        let x = 3;
        x + 1 // this is an expression, returns a value
    };

    another_function(y, "pickles");

    let y: i32 = {
        fn five() -> i32 {
            9
        }

        five()
    };

    another_function(y, "paprikas");

    let y = calculate_something(5, 6);
    another_function(y, "pickels and paprikas");

    let y = calculate_something(6, 5);
    another_function(y, "tomatoes and paprikas");
}

fn calculate_something(x: i32, y: i32) -> i32 {
    if x > y {
        return x - y;
    }

    x + y
}

fn another_function(x: i32, y: &str) {
    println!("Another function has {x} {y}");
}
