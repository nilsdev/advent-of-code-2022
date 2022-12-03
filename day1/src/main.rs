use std::fs; 
fn main() {
    let input_content = fs::read_to_string("./input")   // reading input file to input_content : String
        .expect("Error reading input file"); 

    let mut cal_vec : Vec<i32> = Vec::new();            // vector to store the elf / caloties list in 

    cal_vec.push(0);                                    // create first elf

    for line in input_content.lines()  {                // for line in input_content String
        if line.is_empty() {                            // if line is empty 
            cal_vec.push(0);                            // create new elf / push new element on vector
        } else {                                        // else 
            *cal_vec.last_mut().unwrap()                // return mutable pointer to last element of list and derefrence it
                += line.parse::<i32>().unwrap();        // add the new number on top 
        }
    }

    println!("Part 1: {}", 
        cal_vec.iter().max().unwrap()
    );                                                  // print highest number in vector


    let last3_sum : i32 =                               // new i32 var
        cal_vec.as_slice()[cal_vec.len() -3 ..]         // last 3 items of vector to slice ("part" of a list)
        .to_vec()                                       // convert to vector 
        .iter().sum();                                  // get sum of vector

    println!("Part 2: {:?}", last3_sum);                // print sum
}