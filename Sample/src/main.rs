use std::io;
use std::ops::Add;
fn main() {
    // VARS
    let mut x =String::new();
    let mut y =String::new();
    
    println!("Rust--Calulator");
    
    println!("enter x value");
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");

    println!("enter y value");
    io::stdin().read_line(&mut y)
        .expect("Failed to read line");

    let _answer =x + y;   
    
    println!("{}",_answer);
    

}