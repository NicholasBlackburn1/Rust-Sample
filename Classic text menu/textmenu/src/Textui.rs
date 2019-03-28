extern crate input_stream;
mod test;
mod run;
use std::io;
use input_stream::InputStream;
use test::selftest;
use run::run;

pub fn input(){
      println!("Rust--Menu");
      println!("1: run Self-test");
      println!("2: run Run");
    let stdin = io::stdin();
    let mut input = InputStream::new(stdin.lock());
    let integer: i32 = input.scan().expect("An integer");

    if (integer == 1) {
      println!("Running self test");
      selftest();
    }
    if (integer == 2){
      println!("Runing run");
      run();
    }
}