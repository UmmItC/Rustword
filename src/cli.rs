use std::io::{self, Write};
use termion::color;
use crate::password_generator::PasswordGenerator;

pub struct Cli {
    name: String,
    password_tool: PasswordGenerator,
}

impl Cli {
    pub fn new(name: String) -> Self {
        Cli {
            name,
            password_tool: PasswordGenerator::new(),
        }
    }

    pub fn run(&mut self) {
        let mut input = String::new();
        loop {
            print!("{}{} > {}", color::Fg(color::Blue), self.name, color::Fg(color::Reset));
            io::stdout().flush().expect("Failed to flush stdout");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let trimmed_input = input.trim();

            let mut parts = trimmed_input.split_whitespace();
            let command = parts.next();

            match command {
                Some("gen") => {
                    let password_length_str = parts.next().unwrap_or("12");
                    let password_length: usize = match password_length_str.parse() {
                        Ok(len) => len,
                        Err(_) => {
                            println!("{}Invalid password length. Using default length 12.{}", color::Fg(color::Red), color::Fg(color::Reset));
                            12
                        }
                    };
                    let password = self.password_tool.generate(password_length);
                    println!("{}Generated Password: {}{}", color::Fg(color::Green), password, color::Fg(color::Reset));
                }
                Some("exit") => {
                    println!("{}Exiting Rustword...{}", color::Fg(color::Green), color::Fg(color::Reset));
                    break;
                }
                _ => {
                    println!("{}Unknown command. Available commands: gen, exit{}", color::Fg(color::Red), color::Fg(color::Reset));
                }
            }
        }
    }
} 