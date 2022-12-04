use std::fs; 
fn main() {
    // reading input file to input_content : String
    let input_content = fs::read_to_string("./input") 
        .expect("Error reading input file"); 

    // vector to store the elf / caloties list in 
    let mut calories : Vec<i32> = Vec::new();            

    // create first elf
    calories.push(0);                                    

    // sort colories in reverse and safe it to a new vector named sorted 
    // for line in input_content String
    for line in input_content.lines()  {                
        if line.is_empty() {                            
            // create new elf / push new element on vector
            calories.push(0);                           
        } else {                                        
            // return mutable pointer to last element of list and derefrence it
            *calories.last_mut().unwrap()                
                // add the new number on top 
                += line.parse::<i32>().unwrap();        
        }
    }

    // print highest number in vector
    println!("Part 1: {}", 
        calories.iter().max().unwrap()
    );                                                   

    // vector to store the top 3 elfes in
    let mut sorted = calories.to_owned();                          

    sorted.sort_by(|a, b| b.cmp(a));                    
    println!("Part 2: {}", 
        sorted[..3].iter().sum::<i32>()               
    );
}