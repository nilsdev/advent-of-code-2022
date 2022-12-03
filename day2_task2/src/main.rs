use std::fs; 

fn main() {
    println!("Hello, world!");
    let input_content = fs::read_to_string("./input")
        .expect("Error reading input file"); 

        // Points 
        // A Rock       X       1
        // B Paper      Y       2
        // C Scissors   Z       3
        // Lost 0; Draw 3; Win 6
        let mut result : i32 = 0; 

        for line in input_content.lines() {
            let moves = line.split(" "); 
            let vec = moves.collect::<Vec<&str>>(); 

            match vec.as_slice() {
                ["A", "X"] => result += 3 + 0,
                ["A", "Y"] => result += 1 + 3, 
                ["A", "Z"] => result += 2 + 6,

                ["B", "X"] => result += 1 + 0,
                ["B", "Y"] => result += 2 + 3,
                ["B", "Z"] => result += 3 + 6,

                ["C", "X"] => result += 2 + 0,
                ["C", "Y"] => result += 3 + 3,
                ["C", "Z"] => result += 1 + 6,
                _ => todo!() 
            }

            // lost / draw / win logic
        }
        println!("Score: {}", result)
}