use std::fs;
fn fully_contained(range1: &Vec<i32>, range2: &Vec<i32>) -> bool {
    range1[0] >= range2[0] && range1[1] <= range2[1]
}
fn main() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");

    let mut result = 0;
    for line in input.lines() {
        let team_range: Vec<&str> = line.split(",").collect();

        let first_elf: Vec<i32> =
            team_range[0].split("-").map(|s| s.parse().unwrap()).collect();

        let second_elf: Vec<i32> =
            team_range[1].split("-").map(|s| s.parse().unwrap()).collect();

        if first_elf == second_elf {
            result += 1;
        } else {
            if fully_contained(&first_elf, &second_elf) || fully_contained(&second_elf, &first_elf) {
                result += 1; 
            }
        }

    }
    println!("Part1: {}", result);
}