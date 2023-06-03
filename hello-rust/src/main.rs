use ferris_says::say;
use std::io::{stdout, BufWriter};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn whileing_function(i: i32) -> i32 {
    let mut a = 0;

    while a < i {
        println!("a = {:?}", a);
        a += 1;
    }

    return a;
}

fn looping_function(i: i32) -> i32 {
    let mut a = 0;

    loop {
        println!("a = {:?}", a);

        if a == i {
            break;
        }
        a += 1;
    }

    return a;
}

fn main() {
    let stdout = stdout();
    let message = String::from("Hello there");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();

    say(add(4, 2).to_string().as_bytes(), width, &mut writer).unwrap();

    println!("Hello, world!");
    println!("4 + 2 = {:?}", add(4, 2));

    if add(4, 3) == 6 && add(4, 2) == 6 {
        println!("4 + 2 = 6");
    } else {
        println!("4 + 2 != 6");
    }

    looping_function(32);
    whileing_function(6);
}
