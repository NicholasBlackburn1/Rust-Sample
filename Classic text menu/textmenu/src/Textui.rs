extern crate std;
mod test;
use std::prelude::*;
use test::selftest;

pub fn input(){
    
    let mut reader = io::stdin();
    let input = reader.read_line().ok().expect("Failed to read line");
    let input_num: Option<int> = from_str(input.as_slice().trim());

      println!("Please Enter your choose:");
      println!("1: Start_Self_Test");
      println!("2: Start_program");
      println!("3: QUIT");

}
     