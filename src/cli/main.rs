#[path = "../lib.rs"]
mod lib;

use std::env::{self};

fn main() {
    lib::test();

    // Get args
    let args: Vec<String> = env::args().collect();

    // Put args to variables
    let command: &String = &args[1];
    let option: &String = &args[2];
    let arg: &String = &args[3];

    println!("command: {}", command);
    println!("option: {}", option);
    println!("argument: {}", arg);
}
