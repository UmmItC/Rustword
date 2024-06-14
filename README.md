# Rustword

Rustword is a simple Terminal style password generator written in Rust. It allows you to generate random passwords of custom lengths quickly.

## Features

- Generate random passwords of custom lengths.
- Simple command-line interface.
- Colorful output for improved readability.

## Usage

1. Clone the repository:

    ```bash
    git clone https://codeberg.org/UmmIt/Rustword.git
    ```

2. Navigate to the project directory:

    ```bash
    cd Rustword
    ```

3. Compile and run the program using Cargo:

    ```bash
    cargo run
    ```

4. Follow the on-screen instructions to generate passwords.

## Commands

- `gen <length>`: Generates a random password of the specified length. If no length is provided, the default length is 12.
- `exit`: Exits the Rustword program.

## Dependencies

- [rand](https://crates.io/crates/rand): A crate for random number generation in Rust.
- [termion](https://crates.io/crates/termion): A pure Rust library for controlling the terminal.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
