use std::io;
use std::collections::HashMap;

fn main() {
    let mut user_str = String::new();
    
    println!("Please enter a string");
    
    io::stdin().read_line(&mut user_str)
        .expect("Failed to read user input");

    user_str = user_str.trim().to_string();

    println!("You entered: {}", user_str);
    println!("In pig latin: {}", to_pig_latin(&user_str));
}

fn to_pig_latin(st: &String) -> String { 
    let vowels: HashMap<char, i32> = 
        [('a', 1),
         ('e', 1),
         ('i', 1),
         ('o', 1),
         ('u', 1)]
         .iter().cloned().collect();
    let hay: String = String::from("-hay");
    let pig_str: String = String::from("-");
    let str_copy = st.clone();

    // Check if st starts with a vowel...
    if let Some(ch) = st.chars().next() {
        if let Some(num) = vowels.get(&ch) {
            return str_copy + &hay
        }
    };
    
    for (index, ch) in st.chars().enumerate() {
        if let None = vowels.get(&ch) {
           let pig_str = pig_str + &(ch.to_string() + "ay");
            return st[1..].to_string() + &pig_str 
        };
    }
   
    "".to_string()
}
