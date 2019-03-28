mod test;
mod run;
use std::io::{stdin,stdout,Write};
use std::process::exit;
use test::selftest;
use run::run;

fn input(){

    let mut _input=String::new();
    let _=stdout().flush();

      print!("Please Enter your choose:");
      print!("1: Start_Self_Test");
      println!("2: Start_program");
      println!("3: QUIT");
     
    io::stdin::().read_line(&mut _input).expect(“error: unable to read user input”);

     match _input{
         1 = selftest();
         2 = run();  
         3 = exit(code: i32)
         }
     

}