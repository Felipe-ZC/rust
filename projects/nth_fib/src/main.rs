use std::io;

const FIB_ZERO: i64 = 0;
const FIB_ONE: i64 = 1;

fn main() {
    println!("Please enter a sequence number.");
    
    // Create a new mutable string
    let mut nth_fib = String::new();

    // Read line into mutable string (pass by reference)
    io::stdin().read_line(&mut nth_fib)
        .expect("Failed to read line");

    // Shadow string value to int.
    /*
       REMEMBER: Parse is used to interpert multiple numeric values, 
       therefore we must explicitly mention the type and check the 
       returned Result. To check the returned Result object, we must 
       use the match construct to correctly choose the right Result. 
    */
    let nth_fib: i64 = match nth_fib.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number! Please enter a number!");
            return;
        },
    };

    println!("The {} fibonacci number is: {}", nth_fib, fib(nth_fib));
}

fn fib(n: i64) -> i64 {
    if n == 0 {
        FIB_ZERO
    }
    else if n == 1 {
        FIB_ONE
    }
    else {
        fib(n-2) + fib(n-1)
    }
}
