use std::io;
use std::io::Write;
use std::collections::HashMap;

// Use &str more?

fn main() {
    
    const exit: &str = "q";
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    let mut user_in = String::new();
    
    println!("Welcome to Z-Corp department database!");

    while user_in.trim() != exit {
        print_menu();    
        
        user_in = "".to_string();
        
        io::stdin().read_line(&mut user_in)
            .expect("Could not read user input!");
        
        match user_in.trim().parse::<u32>() {
            Ok(num) => get_ops(num, &mut departments),
            Err(String) => {
                if let exit = user_in.trim() {
                    println!("Goodbye!");
                }
                else {
                    println!("Invalid option!");
                }
            }
        };

        // Clear terminal
        //print!("\x1B[2J");
        // print!("{}[2J", 27 as char);
    }
    
}

fn print_menu() {
    println!("Please select an operation:\n1) Add employee to a deparment\n2) Retrieve list of employees per deparment\n3) Print all employees names in a department\n4) Remove employee from a deparment");
}

fn get_ops(op: u32, map: &mut HashMap<String, Vec<String>>) {
    match op {
        1 => add_employee(map),
        2 => print_all(map),
        3 => print_dept(map),
        4 => remove_employee(map),
        _ => ()
    };   
}


fn print_dept(depts: &mut HashMap<String, Vec<String>>) { 
    println!("Please enter the name of the department you wish to view");
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)
            .expect("Could not read user input!");
    
    // .get() expects a reference!
    if let Some(names) = depts.get(&input.trim().to_string()) {
        for name in names {
            println!("{}", name);
        }
    }

}

fn add_employee(depts: &mut HashMap<String, Vec<String>>) {
    println!("Please enter the name of the employee followed by the name of the department\nExample: Sally Engineering");
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)
            .expect("Could not read user input!");

    let mut args = input.trim().split(" ");

    if let (Some(name), Some(dept)) = (args.next(), args.next()) {
        if let Some(names) = depts.get_mut(dept) {
            names.push(name.to_string());
        }
        else {
            depts.insert(dept.to_string(), vec![name.to_string()]);
        }
    }

}
    
fn print_all(depts: &mut HashMap<String, Vec<String>>) {
    // .keys() RETURNS A REFERENCE!!!!!
    //let mut depts_copy: Vec<&String>  = depts.keys().collect(); 
    for (dept, names) in depts {
        println!("--------{}--------", dept);
        names.sort();
        for name in names {
            println!("{}", name);
        }
    }
}

fn remove_employee(depts: &mut HashMap<String, Vec<String>>) {

}
