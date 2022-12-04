use std::{fs, collections::HashSet}; 
fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let input_content = fs::read_to_string("./input").expect("Error reading input file");

    part1(&input_content, &alphabet); 
    part2(&input_content, &alphabet); 

}

fn part2(input_content : &String, alphabet : &Vec<char>) {
    let mut result : usize = 0; 

    let mut set: HashSet<char> = HashSet::new(); 
    for (index, line) in input_content.lines().enumerate() {
        // let mut three_elves : Vec<char>; 

        if index % 3 == 0 {
            // println!("==="); 
            set = line.chars().collect(); 
        } else {
            let mut missed : Vec<char> = Vec::new(); 
            for letter in set.iter() {
                if !line.contains(*letter) {
                    missed.push(*letter); 
                }
            }

            for letter in missed.iter() {
                set.remove(letter); 
            }
        }

        // println!("{} | {}", index, line); 

        if index % 3 == 2 {
            let value : usize = get_letter_value(set.iter().max().unwrap(), alphabet); 
            // println!("{:?} -> {}", set, value); 
            result += value; 
        }
    }
    println!("Part2: {}", result); 
}

fn get_letter_value(letter : &char, alphabet : &Vec<char>) -> usize {
    let value = alphabet.iter().position(|&x| x == *letter).unwrap() + 1; 
    value
}

fn part1(input_content : &String, alphabet : &Vec<char>) {
    let mut result : usize = 0; 
    for line in input_content.lines() {
        let split = line.split_at(line.chars().count() / 2); 
        for letter in split.0.chars() {
            if split.1.contains(letter) {
                let value : usize = get_letter_value(&letter, alphabet); 
                result += value; 
                break;
            }
        }
    }
    println!("Part1: {}", result); 
}