#![allow(unused)]
use md5;

//yzbqklnj
fn main() {
    let mut input = "yzbqklnj".to_string();
    for i in 100000..9999999 {
        let string = format!("{}{}", input, i);
        let hash = hash(&string);
        if &hash[0..6] == "000000".to_string() {
            println!("Hash found {}", hash);
            println!("Number is {}", i);
            break;
        }
    }
}

fn hash(str: &String) -> String {
    let digest = md5::compute(str.as_bytes());
    return format!("{:x}", digest);
}
