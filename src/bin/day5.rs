use std::fs;

fn main() {
    let file_path = "inputs/day5.txt";
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let ans = part_one(&content);
            println!("part one ans is {}", ans);
            let ans1 = part_two(&content);
            println!("Part Two ans is {}", ans1);
        }
        Err(e) => {
            println!("Error {}", e);
        }
    }
}
fn part_one(content: &String) -> u32 {
    let mut total = 0;
    let restricted_words: Vec<String> = vec![
        "ab".to_string(),
        String::from("cd"),
        String::from("pq"),
        String::from("xy"),
    ];
    let vowels = [
        String::from("a"),
        String::from("e"),
        String::from("i"),
        String::from("o"),
        String::from("u"),
    ];
    for s in content.lines() {
        if restricted_words.iter().any(|w| s.contains(w)) {
            continue;
        }
        let vowel_vec: Vec<char> = s
            .chars()
            .filter(|s| vowels.contains(&s.to_string()))
            .collect();
        let no_of_vowels = vowel_vec.len();
        if no_of_vowels < 3 {
            continue;
        }
        let double: Vec<char> = s.chars().collect();
        let mut has_double = false;

        for d in 0..double.len() - 1 {
            if double[d] == double[d + 1] {
                has_double = true;
                break;
            }
        }

        if has_double {
            total += 1;
        }
    }
    return total;
}

fn part_two(content: &String) -> u32 {
    let mut total: u32 = 0;
    for line in content.lines() {
        let char: Vec<char> = line.chars().collect();
        let mut is_one_true = false;
        let mut is_two_true = false;
        for i in 0..line.len().saturating_sub(1) {
            let pairs = &line[i..(i + 2)];
            if line[(i + 2)..].contains(pairs) {
                is_one_true = true;
                break;
            }
        }
        for i in 0..line.len().saturating_sub(2) {
            if char[i] == char[i + 2] {
                is_two_true = true;
                break;
            }
        }
        if is_one_true && is_two_true {
            total += 1;
        }
    }

    return total;
}
