use std::io;

fn main() {
    println!("Please enter a temperature followed by a whitespace and F or C\nEx: 32 F (32 degrees Fahrenheit)");
    
    let mut user_in = String::new();

    io::stdin().read_line(&mut user_in)
        .expect("Failed to read line!");

    let mut split = user_in.split(" ");

    let user_temp: f64 = match split.next() {
        Some(val) => {
            match val.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Could not parse user input");
                    0.0
                },
            }
        },
        None => {
            println!("Not a number!"); 
            0.0
        },
    };

    match split.next() {
        Some(val) => {
            println!("val = {}", val);
            if val.trim() == "F" {
                println!("{} F <---> {} C", user_temp, convert_to_cel(user_temp));
            }
            else if val.trim() == "C" { 
                println!("{} C <---> {} F", user_temp, convert_to_far(user_temp));
            }
            else {
                println!("Invalid reading");
            }
        },
        None => {
            println!("Invalid reading");
        }
    }
}

fn convert_to_cel(temp: f64) -> f64 {
    (temp - 32.0) * (5.0/2.0)
}

fn convert_to_far(temp: f64) -> f64 {
    (temp * (9.0/5.0)) + 32.0
}
