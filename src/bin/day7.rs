#![allow(unused)]
use std::fs;
fn main() {
    let path = "inputs/day7.txt";
    match fs::read_to_string(path) {
        Ok(content) => {
            println!("{}", content);
        }
        Err(e) => {
            println!("Error {e}");
        }
    }
}
