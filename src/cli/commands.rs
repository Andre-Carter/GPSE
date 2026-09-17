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
            "calc" => {
                cli_calc();
            }

            "help" => {
                println!("Commands:");
                println!("calc");
                println!("help");
                println!("exit");
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