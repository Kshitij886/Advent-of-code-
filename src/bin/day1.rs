#![allow(unused)]

use std::fs;
use std::io;

fn main() {
    let file_path = "../../inputs/day1.txt";
    match fs::read_to_string(file_path) {
        Ok(content) => {
            match part1(&content) {
                Ok(ans) => {
                    println!("Part1 {}", ans);
                },
                Err(e) => {
                    println!("{}", e);
                }
            };
            match part2(&content) {
                Ok(ans ) => {
                    println!("Part2 ans {}", ans);
                },
                Err(e) => {
                    println!("{}", e);
                }
                
            }

        }
        Err(e) => {
            println!("Error {}", e);
        }
    };
}

fn part1(string: &String) -> Result<i32, &'static str> {
    let mut position: i32 = 0;
    for c in string.chars() {
        if c == '(' {
            position += 1;
        } else if c == ')' {
            position -= 1;
        } else {
            position = position;
        }
    }
    return Ok(position);
}
fn part2(string: &String) -> Result<u32, &'static str> {
    let mut position : i32 = 0;
    let mut index : u32 = 0;
    for c in string.chars() {
        index += 1;
        if c == '(' {
            position += 1;
        } else if c == ')' {
            position -= 1;
        } else {
            position = position;
        }
        if position == -1 {
            return Ok(index);
        }
    }
    return Err("Not found");
}
