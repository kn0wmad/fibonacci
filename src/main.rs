#![allow(non_snake_case)]

use std::io;

//create Fibonacci Sequence and return a chosen number 'f' from within
fn fib(f: u32) -> u32 {
    match f {
        0 => 1,
        1 => 1,
        _ => fib(f - 1) + fib(f - 2),
    }
}

fn main() {
    //initiate program with title
    println!("Fibonacci Numbers");

    //ask for and read user input, trim and convert to int
    //clarify if unsuitable and allow exit
    loop {
        let mut userInput = String::new();
        println!("Please select a position in the Fibonacci Sequence:");
        io::stdin().read_line(&mut userInput).expect("Failed to read from stdin");

        if userInput.trim() == "exit" {
            break;
        }
        
        let userInput: u32 = match userInput.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
            };

        //run fib fucntion with user input and return the value from the Sequence
        println!("The Fibonacci number is {}", fib(userInput));
    };
}
