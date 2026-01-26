#![allow(unused)]

use std::fs;

fn main() {
    let file_path = "inputs/day3.txt";
    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("{}", content);
        },
        Err(e) => {
            println!("Error : {}", e);
        }
    }

}