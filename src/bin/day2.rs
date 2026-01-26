#![allow(unused)]
use std::{fs, ptr::read};

fn main() {
    let file_path = "inputs/day2.txt";
    match fs::read_to_string(file_path) {
        Ok(content) => {
            match part1(&content) {
                Ok(ans) => {
                    println!(" part one ans: {}", ans);
                },
                Err(e) => {
                    println!("{}",e );
                }
            };
            match part2(&content) {
                Ok(ans) => {
                    println!(" part two ans: {}", ans);
                },
                Err(e) => {
                    println!("{}",e );
                }
            }
        },
        Err(e) => {
            println!("Error {}", e);
        }
    }
}

fn part1(string: &String) -> Result<u32, &'static str>{
    let mut total_area : u32 = 0;
    let mut first_side_area : u32 = 0;
    let mut second_side_area: u32 = 0;
    let mut third_side_area: u32 = 0;
    for s in string.lines() {
        let number : Vec<u32> = s.split('x').filter_map(|sub| sub.parse().ok()).collect();
        first_side_area = number[0] * number[1];
        second_side_area = number[1] * number[2];
        third_side_area = number[2] * number[0];
        total_area = total_area + 2 * first_side_area + 2 * second_side_area + 2 * third_side_area + first_side_area.min(second_side_area).min(third_side_area);
    }
    return Ok(total_area)
}
fn part2(string: &String) -> Result<u32, &'static str> {
    let mut total_length : u32 = 0;
    for s in string.lines() {
        let number : Vec<u32> = s.split('x').filter_map(|sub| sub.parse().ok()).collect();
        let min_number = number[0].min(number[1]).min(number[2]);
        let next_min = if min_number == number[0] {
            number[1].min(number[2])
        } else if min_number == number[1] {
            number[0].min(number[2])
        } else {
            number[0].min(number[1])
        };  
        total_length = total_length + (number[0] * number[1] * number[2]) + 2*min_number + 2 * next_min;
    }
    return Ok(total_length)

}