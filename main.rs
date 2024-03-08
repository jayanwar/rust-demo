use std::io::{self, BufRead};

mod module;

fn main() {
    println!("hello geez - how many fibs you want?");

    let stdin = io::stdin();

    let mut fibn: u64;

    for line in stdin.lock().lines() {
        if line.is_ok() {
            fibn = line.unwrap().parse().unwrap();

            println!("your fibs geez");

            module::fibonacci(fibn, 0, 1);

            println!("you want any more?");
        }
    }
}  