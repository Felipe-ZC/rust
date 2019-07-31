/*
   The #[derive(Debug)] is an annotation that allows Rust to print debug information 
   for a non primitive type.
*/


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    
    println!("The area of the rectangle is {} square pixels.", 
             rect1.area());

    //println!("rect1 is {:#?}", rect1); // Used to print debug info for struct
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
