
use std::collections::HashMap;
const vowels: HashMap<&str, i32> = 
    [("a", 1),
     ("e", 1),
     ("i", 1),
     ("o", 1),
     ("u", 1)]
     .iter().cloned().collect();


fn main() {
    println!("Hello, world!");
}

fn to_pig_latin(st: &String) -> String {
    // Check if st starts with a vowel...
    if let Some(ch) = st.chars().next() {
        if let Some(num) = vowels.get(&ch) {
            return st + "-hay";
        }
    };
    
    /*
    for ch in st.chars() {
        if let None = vowels.get(&ch) {
           print!("Swag!"); 
        }
   }*/
}
