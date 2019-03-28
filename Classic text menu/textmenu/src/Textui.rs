extern crate input_stream;
mod test;
use std::io;
use input_stream::InputStream;
use test::selftest;

pub fn input(){
      println!("pL");
    
    let stdin = io::stdin();
    let mut input = InputStream::new(stdin.lock());
    let integer: i32 = input.scan().expect("An integer");

    if (integer == 1) {
      println!("hello 1")
    }
}