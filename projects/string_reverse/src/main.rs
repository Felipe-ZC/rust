use std::io;

fn main() {
    println!("Please enter a string: ");
    
    let mut user_in = String::new();

    io::stdin().read_line(&mut user_in)
        .expect("Failed to read line");   
    
    user_in = user_in.trim().to_string();

    println!("{} reversed is {}", user_in, reverse(&user_in));
}

fn reverse(user_str: &String) -> String {
    
    let mut rev_str = String::new();
    let mut copy_str = String::from(user_str);

    while !copy_str.is_empty() {
        if let Some(ch) = copy_str.pop() {
            rev_str.push(ch);
        }
    }
    
    rev_str
}
