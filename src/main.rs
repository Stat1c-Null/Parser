use std::fs::File;
use std::io::{self, Read};

mod parser;
use parser::parser::Parser;

fn run(label: &str, code: &str) {
    println!("--- {} ---", label);
    println!("{}", code);
    println!();
    match Parser::new(code.to_string()).and_then(|mut p| p.parse()) {
        Ok(()) => {
            println!("THE CODE SAMPLE IS ACCEPTED-");
            println!("(THE LANGUAGE IS ACCEPTED BY THE GRAMMAR)");
        }
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let default_code = concat!(
        "x = 10\n",
        "\n",
        "if x < 0:\n",
        "    print(\"x is negative\")\n",
        "elif x == 0:\n",
        "    print(\"x is zero\")\n",
        "else:\n",
        "    print(\"x is positive\")\n",
    );

    let buggy_code = concat!(
        "x = 10\n",
        "\n",
        "if x < 0\n",
        "    print(\"x is negative\")\n",
        "elif x == 0:\n",
        "    print(\"x is zero\")\n",
        "else:\n",
        "    print(\"x is positive\")\n",
    );

    println!("Python If-Else statement Parser");
    println!("Select an option:");
    println!(" 1) Analyze default code");
    println!(" 2) Analyze code from file");
    println!(" 3) Analyze default code with intentional syntax error");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let choice = input.trim();

    match choice {
        "1" => run("Default code", default_code),
        "2" => {
            println!("Enter file path:");
            let mut path = String::new();
            io::stdin()
                .read_line(&mut path)
                .expect("Failed to read path");
            let path = path.trim();
            match File::open(path) {
                Ok(mut f) => {
                    let mut contents = String::new();
                    if let Err(e) = f.read_to_string(&mut contents) {
                        println!("Error reading file: {}", e);
                        return;
                    }
                    run(&format!("File: {}", path), &contents);
                }
                Err(e) => println!("Error opening file '{}': {}", path, e),
            }
        }
        "3" => run("Default code with intentional error", buggy_code),
        _ => println!("Unknown option"),
    }
}
