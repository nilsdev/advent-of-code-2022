use std::fs; 
fn main() {
    // reading input file to input_content : String
    let input_content = fs::read_to_string("./input") 
        .expect("Error reading input file"); 

    // vector to store the elf / caloties list in 
    let mut calories : Vec<i32> = Vec::new();            

    // create first elf
    calories.push(0);                                    

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
    let mut top_three : Vec<i32> = Vec::new();          

    for _ in 0..3 {
        // get current max value
        let max = *calories.iter().max().unwrap();       
        // push max value on top_three vector
        top_three.push(max);                            
        // find next largest elf by: 
        calories.swap_remove(                           
            // iterating over calories Vector
            calories.iter()                             
            // find index of current max value 
            .position(|&x| x == max)                    
            .unwrap()                                   
        // and removing current max elf 
        ); 
    }

    // print highest number in vector
    println!("Part 2: {:?}", 
        top_three.iter().sum::<i32>()
    );                                                  
}