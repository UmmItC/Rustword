use rand::prelude::*;
use std::io::{self, Write};
use termion::color;

fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@#$%^&*()-_=+";

    let mut rng = thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}

fn main() {
    let mut input = String::new();
    let name = String::from("Rustword");
    loop {
        print!("{}{} > {}", color::Fg(color::Blue), name, color::Fg(color::Reset));
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed_input = input.trim();

        let mut parts = trimmed_input.split_whitespace();

        match parts.next() {
            Some("gen") => {
                let password_length_str = parts.next().unwrap_or("12"); // Default password length is 12
                let password_length: usize = match password_length_str.parse() {
                    Ok(len) => len,
                    Err(_) => {
                        println!("{}Invalid password length. Using default length 12.{}", color::Fg(color::Red), color::Fg(color::Reset));
                        12
                    }
                };
                let password = generate_password(password_length);
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
