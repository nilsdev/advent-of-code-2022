use std::fs; 
fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let input_content = fs::read_to_string("./input").expect("Error reading input file");

    let mut result : usize = 0; 
    for line in input_content.lines() {
        let split = line.split_at(line.chars().count() / 2); 
        for letter in split.0.chars() {
            if split.1.contains(letter) {
                let value = alphabet.iter().position(|&x| x == letter).unwrap() + 1; 
                println!("Found: {} + {}", letter, value); 
                result += value; 
                break;
            }
        }
    }
    println!("Part1: {}", result); 
}
