#![allow(unused)]
use std::{
    collections::{HashMap, hash_map::Values},
    fs,
};

fn main() {
    let path = "inputs/day6.txt";
    match fs::read_to_string(path) {
        Ok(content) => {
            let ans = part_one(&content);
            println!("Part one ans {}", ans);
            let ans1 = part_two(&content);
            println!("Part two ans {}", ans1);
        }
        Err(e) => {
            println!("Error {}", e);
        }
    }
}

fn part_one(content: &String) -> u32 {
    let mut turn_on = HashMap::<(u32, u32), String>::new();
    for line in content.lines() {
        let chars: Vec<&str> = line.split(" ").collect();
        if chars[0] == "turn" && chars[1] == "on" {
            let start: Vec<u32> = chars[2]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let end: Vec<u32> = chars[4]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            for i in start[0]..end[0] + 1 {
                for j in start[1]..end[1] + 1 {
                    turn_on.insert((i, j), String::from("turn on"));
                }
            }
        }
        if chars[0] == "turn" && chars[1] == "off" {
            let start: Vec<u32> = chars[2]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let end: Vec<u32> = chars[4]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            for i in start[0]..end[0] + 1 {
                for j in start[1]..end[1] + 1 {
                    if let Some(value) = turn_on.get(&(i, j)) {
                        turn_on.remove(&(i, j));
                    }
                }
            }
        }
        if chars[0] == "toggle" {
            let start: Vec<u32> = chars[1]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let end: Vec<u32> = chars[3]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            for i in start[0]..end[0] + 1 {
                for j in start[1]..end[1] + 1 {
                    if let Some(value) = turn_on.get(&(i, j)) {
                        turn_on.remove(&(i, j));
                    } else {
                        turn_on.insert((i, j), String::from("turn on"));
                    }
                }
            }
        }
    }
    return turn_on.len() as u32;
}

fn part_two(content: &String) -> u32 {
    let mut grid = HashMap::<(u32, u32), u32>::new();

    for line in content.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let (start_str, end_str) = if words[0] == "toggle" {
            (words[1], words[3])
        } else {
            (words[2], words[4])
        };
        let start: Vec<u32> = start_str.split(',').map(|s| s.parse().unwrap()).collect();
        let end: Vec<u32> = end_str.split(',').map(|s| s.parse().unwrap()).collect();

        for i in start[0]..=end[0] {
            for j in start[1]..=end[1] {
                let entry = grid.entry((i, j)).or_insert(0);
                match words[0] {
                    "toggle" => *entry += 2,
                    "turn" => {
                        if words[1] == "on" {
                            *entry += 1;
                        } else {
                            if *entry > 0 {
                                *entry -= 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    grid.values().sum()
}
