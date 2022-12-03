use std::fs; 
fn main() {
    let input_content = fs::read_to_string("./input")   // reading input file to input_content : String
        .expect("Error reading input file"); 

    let mut calories : Vec<i32> = Vec::new();            // vector to store the elf / caloties list in 

    calories.push(0);                                    // create first elf

    for line in input_content.lines()  {                // for line in input_content String
        if line.is_empty() {                            // if line is empty 
            calories.push(0);                            // create new elf / push new element on vector
        } else {                                        // else 
            *calories.last_mut().unwrap()                // return mutable pointer to last element of list and derefrence it
                += line.parse::<i32>().unwrap();        // add the new number on top 
        }
    }

    println!("Part 1: {}", 
        calories.iter().max().unwrap()
    );                                                   // print highest number in vector

    let mut top_three : Vec<i32> = Vec::new();          // vector to store the top 3 elfes in

    for _ in 0..3 {
        let max = *calories.iter().max().unwrap();       // get current max value
        top_three.push(max);                            // push max value on top_three vector
        calories.swap_remove(                            // find next largest elf by: // using swap_remove() to avoid O(n) performance of remove()
            calories.iter()                              // iterating over calories Vector
            .position(|&x| x == max)                    // find index of current max value 
            .unwrap()                                   // and removing current max elf
        );  
    }

    println!("Part 2: {:?}", 
        top_three.iter().sum::<i32>()
    );                                               // print highest number in vector
}