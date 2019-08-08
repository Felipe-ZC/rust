use rand::Rng; 
use std::collections::HashMap;
use std::io;


fn main() {
    println!("Please enter an integer range (start, end) separated by a whitespace, for example: 2 20");
    
    let mut user_in = String::new();

    io::stdin().read_line(&mut user_in)
        .expect("Error! Could not read line.");
    
    // Map each string in user_in to the parse function for i32's.
    // Map returns an iterator where each value is a Result (Some or None)
    // wrapped in an Option (Ok, None). This is because map applies the parse 
    // function to each element in user_in, map returns an iterator which is a
    // result object, parse returns an options object.
    let mut parts = user_in.trim().split_whitespace()
        .map(|word| word.parse::<i32>());
   
    let range = match (parts.next(), parts.next()) {
        (Some(Ok(num1)), Some(Ok(num2))) => (num1, num2),
        _ => (0,0)
    };

    let numbers = generate_int_measures(range);

    println!("Measures: {:?}", &numbers);
    println!("Mean: {}", calculate_mean(&numbers));
    println!("Mode: {}", calculate_mode(&numbers));
    println!("Median: {}", calculate_median(&numbers));
}

// Creates and returns an integer vector of 
// a random size with random integer values.  
fn generate_int_measures(range: (i32, i32)) -> Vec<i32> {
    let size = rand::thread_rng().gen_range(range.0, range.1) as usize;
    let mut result_vec: Vec<i32>  = Vec::new(); 
    
    while result_vec.len() < size {
        result_vec.push(rand::thread_rng().gen_range(range.0, range.1));
    }
    
    result_vec.sort();
    result_vec
}

fn calculate_median(measures: &Vec<i32>) -> i32{
    let median_index = measures.len() as i32 as i32 / 2;
    measures[median_index as usize]
}

fn calculate_mode(measures: &Vec<i32>) -> i32 {
    let mut numbers = HashMap::new(); 
    let mut mode = 0;
    let mut mode_max_count = 0;

    for num in measures {
        // Checks if num is present in hash map
        // if not, it inserts 0 as the value for 
        // num. Increase count by 1 if present.
        let count = numbers.entry(num).or_insert(0);
        *count += 1; // Deref. here because or_insert returns mutable ref.

        if mode_max_count < *count {
            mode = *num;
            mode_max_count = *count;
        }
    }

    mode
}

fn calculate_mean(measures: &Vec<i32>) -> f32 {
    let mut sum = 0.0;
    
    for num in measures {
        // Deref. integer pointer and cast to float
        sum += *num as f32;
    }

    sum / (measures.len() as f32)
}

