use std::fs::File;
use std::io::{self, Read};

fn main() {
    let default_code = "x = 10
        if x < 0:
            print('x is negative')
        elif x == 0:
            print('x is zero')
        else:
            print('x is positive')";
    
    
    println!("Python If-Else statement Parser");
    println!("Select an option:\n 1) Analyze Default code\n 2) Analyze code from file (TODO)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    if input == "1" {
        println!("Default code parsing selected");
        println!("Default code: \n{}\n", default_code);
    } else if input == "2" {
        println!("Code from file parsing selected");
    } else {
        println!("Unknown option");
    }

}
