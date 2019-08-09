use std::io;
use std::collections::HashMap;

fn main() {
    
    let mut depts: HashMap<String, String> = HashMap::new();
    let mut user_in = String::new();
    
    while user_in != "q\n" {
        print_menu();    
        
        io::stdin().read_line(&mut user_in)
            .expect("Could not read user input!");
        
        println!("You entered: {}", &user_in);
    }
    
}

fn print_menu() {
    println!("Welcome to Z-Corp department database!\nPlease select an operation:\n1) Add employee to a deparment\n2) Retrieve list of employees per deparment\n3) Remove employee from a deparment");
}

fn get_ops(op: i32, map: HashMap<String, String>) {
    match op {
        1 => add_employee(map),
        2 => print_all(map),
        3 => remove_employee(map),
        _ => ()
    };   
}

fn add_employee(depts: HashMap<String, String>) {

}

fn print_all(depts: HashMap<String, String>) {

}

fn remove_employee(depts: HashMap<String, String>) {

}
