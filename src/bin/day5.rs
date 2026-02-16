use std::fs;

fn main() {
    let file_path = "./day5.txt";
    match fs::read_to_string(file_path) {
        Ok(content) => {
            part_one(&content);
        }
        Err(e) => {
            println!("Error {}", e);
        }
    }
}
fn part_one(content: &String) -> String {
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
            break;
        }
        let vowel_vec: Vec<char> = s
            .chars()
            .filter(|s| vowels.contains(&s.to_string()))
            .collect();
        let no_of_vowels = vowel_vec.len();
        if no_of_vowels < 3 {
            break;
        }
        let double =s.chars();
        let mut i: u32 = 0;
        for d in double {
            i += 1;
            if

        }
    }
    return String::from("hello");
}
