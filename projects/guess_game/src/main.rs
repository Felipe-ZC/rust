use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*
   NOTE: Rust has a strong, static type system but also 
   supports type inference.
*/

fn main() {
    
    /* 
       Generates a random number from 1 to 100, 
       thread_rng returns a random number generator
       that is local to the current thread of execution
       and is seeded by the operating system.

       gen_range takes in two number arguments 
       ,upper (exclusive) & lower bounds (inclusive),
       and returns a random number between them.
    */
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        //println!("The secret number is {}", secret_number);
        println!("Guess the number!\nPlease input a number between 1 & 100");
   
        /*
        Rust allows for 'shadowing' of variables. This feature 
        is often useful when we want to convert a value to a 
        different type. 'Shadowing' lets us reuse the variable
        name guess instead of creating another unique name. 

        trim() eliminates any leading or trailing whitespace in 
        a string.

        parse() interprets a string as a numerical value. This method
        can be used on a variety of numerical types, therefore we must
        explicitly mention the type (u32). The colon (:) tells Rust that
        what type the value of 'guess' will be . 
        */
        let mut guess = String::new();
        println!("Please enter your guess.");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // '_' refers to any value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
   
        println!("You guessed: {}", guess);
    
        // Since guess is now a u32, Rust will infer that secret_number is also a u32.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
        }
    }
        
    
}

