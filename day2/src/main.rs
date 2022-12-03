use std::fs;

fn main() {
    println!("Hello, world!");
    let input_content = fs::read_to_string("./input").expect("Error reading input file");

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for line in input_content.lines() {
        let moves = line.split(" ");
        let vec = moves.collect::<Vec<&str>>();

        // there has to be a nicer / scaleable way to do this
        match vec.as_slice() {
            ["A", "X"] => part1 += 1 + 3,
            ["A", "Y"] => part1 += 2 + 6,
            ["A", "Z"] => part1 += 3 + 0,

            ["B", "X"] => part1 += 1 + 0,
            ["B", "Y"] => part1 += 2 + 3,
            ["B", "Z"] => part1 += 3 + 6,

            ["C", "X"] => part1 += 1 + 6,
            ["C", "Y"] => part1 += 2 + 0,
            ["C", "Z"] => part1 += 3 + 3,
            _ => todo!(),
        }

        match vec.as_slice() {
            ["A", "X"] => part2 += 3 + 0,
            ["A", "Y"] => part2 += 1 + 3,
            ["A", "Z"] => part2 += 2 + 6,

            ["B", "X"] => part2 += 1 + 0,
            ["B", "Y"] => part2 += 2 + 3,
            ["B", "Z"] => part2 += 3 + 6,

            ["C", "X"] => part2 += 2 + 0,
            ["C", "Y"] => part2 += 3 + 3,
            ["C", "Z"] => part2 += 1 + 6,
            _ => todo!(),
        }
    }
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}