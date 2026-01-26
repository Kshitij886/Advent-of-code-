#![allow(unused)]
use std::{fs, ptr::read};

fn main() {
    let file_path = "inputs/day2.txt";
    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("{}", content);
        },
        Err(e) => {
            println!("Error {}", e);
        }
    }
}