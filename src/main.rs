mod cli;
mod password_generator;

use cli::Cli;

fn main() {
    let mut app = Cli::new("rustword".to_string());
    
    app.run();
}