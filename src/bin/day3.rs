#![allow(unused)]

use std::{ collections::HashMap, fs, hash::Hash };

fn main() {
    let file_path = "inputs/day3.txt";
    match fs::read_to_string(file_path) {
        Ok(content) => {
            match part_one(&content) {
                Ok(no) => {
                    println!("No of Unique houses = {}", no);
                }
                Err(e) => {
                    println!("Error {}", e);
                }
            }
            match part_two(&content) {
                Ok(no) => {
                    println!("No of total Unique houses : {}", no);
                }
                Err(e) => {
                    println!("Error {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error : {}", e);
        }
    }
}

fn part_one(string: &String) -> Result<u32, &'static str> {
    let mut coordinate = HashMap::<(i32, i32), u32>::new();
    let mut index: u32 = 0;
    coordinate.insert((0, 0), 1);
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for c in string.chars() {
        index = index + 1;
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => return Err("Invalid character"),
        }
        *coordinate.entry((x, y)).or_insert(0) += 1;
    }

    return Ok(coordinate.len() as u32);
}
fn part_two(input: &str) -> Result<u32, &'static str> {
    let mut houses: HashMap<(i32, i32), u32> = HashMap::new();

    let mut santa = (0, 0);
    let mut robot = (0, 0);

    *houses.entry((0, 0)).or_insert(0) += 1;

    for (i, c) in input.chars().enumerate() {
        let pos = if i % 2 == 0 {
            &mut santa
        } else {
            &mut robot
        };

        match c {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => return Err("Invalid character"),
        }

        *houses.entry(*pos).or_insert(0) += 1;
    }

    Ok(houses.len() as u32)
}