#![allow(dead_code)]
#![allow(unused)]

use std::fs;

mod loops_and_conditions;
mod ownership;
mod primitive_types;
mod structs;

const CONFIG_FILE: &str = "config.txt";

fn main() {
    let contents = fs::read_to_string(CONFIG_FILE).unwrap();
    println!("..contents: {}", contents);
}
