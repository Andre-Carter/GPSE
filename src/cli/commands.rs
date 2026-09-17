use crate::cli::calculator::cli_calc;
use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

pub fn cli_commands() {
    println!("Commands system online!");
    
    loop {
        let mut command: String = String::new();

        print!("> ");
        read(&mut command);

        let command = command.trim();

        match command {
            "help" => {
                println!("Commands:");
                println!("calc");
                println!("cargo");
                println!("git")
                println!("exit");
            }

            "calc" => {
                cli_calc();
            }

            "cargo" => {
                println!("cargo check");
            }

            "git" => {
                println!("git status");
            }

            "exit" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Unknown command.")
            }
        }
    }

}