#![allow(warnings)]   

use std::io;

fn main() {
    println!("Enter a number to add: ");
    //let fib: u64 = fibonacci(93);

    let mut add = String::new();
    io::stdin()
        .read_line(&mut add)
        .expect("FAILED");
    let number: u64 = add.trim().parse().expect("Not valid");
    add_sequence(number);
}

fn fibonacci(mut fib: u64) -> u64 {
    let mut start: u64 = 0;
    let mut next: u64 = 1;
    let mut last: u64 = start;
    let result: u64 = loop {
        if fib == 0 {
            break start;
        }
        next += last;
        last = start;
        start = next;
        fib -= 1;
    };

    return result;
}

fn add_sequence(range: u64) {
    let mut num = 0;
    for number in 1..range {
        num += number;
        println!("{num}");

    }
}
