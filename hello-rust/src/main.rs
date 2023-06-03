use ferris_says::say;
use std::io::{stdout, BufWriter};

fn add(a:i32, b:i32) -> i32 {
    a + b
}

fn main(){
    let stdout = stdout();
    let message = String::from("Hello there");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();

    say(add(4,2).to_string().as_bytes(), width, &mut writer).unwrap();

}