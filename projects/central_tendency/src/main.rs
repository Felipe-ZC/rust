use rand::Rng; 

fn main() {
    println!("Hello, world!");
    
    println!("Vector: {}", generate_int_measures((0,5)));

   // let integer_measures = vec![2,3,4,1];
}

// Creates and returns an integer vector with random values.  
fn generate_int_measures(range: (i32, i32)) -> Vec<i32> {
    let size = rand::thread_rng().gen_range(range.0, range.1);
    let result_vec: Vec<i32>  = Vec::new(); 
    
    while result_vec.len() < size {
        result_vec.push(rand::thread_rng().gen_range(range.0, range.1));
    }

    result_vec
}

fn get_central_tendency(measures: Vec<i32>) {

}
