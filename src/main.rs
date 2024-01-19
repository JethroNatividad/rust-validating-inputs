use regex::Regex;
use std::io;
use std::io::Write;

// Write a program that prompts for a first name, last name,
// employee ID, and ZIP code. Ensure that the input is valid
// according to these rules:
// • The first name must be filled in.
// • The last name must be filled in.
// • The first and last names must be at least two characters
// long.
// • An employee ID is in the format AA-1234. So, two letters, a hyphen, and four numbers.
// • The ZIP code must be a number.
// Display appropriate error messages on incorrect data.

fn validate_name(name: &str, field_name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err(format!("The {} must be filled in.", field_name));
    }
    Ok(())
}

fn validate_regex(text: &str, regex_pattern: &str, error_message: String) -> Result<(), String> {
    let re: Regex = Regex::new(regex_pattern).unwrap();
    let Some(_caps) = re.captures(text) else {
        println!("no match!");
        return Err(error_message);
    };
    Ok(())
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
