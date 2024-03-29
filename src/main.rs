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

fn validate_regex(text: &String, regex_pattern: &str) -> Result<(), ()> {
    let re: Regex = Regex::new(regex_pattern).unwrap();
    let Some(_caps) = re.captures(&text) else {
        return Err(());
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
    let first_name: String = get_input("Enter the first name: ");
    let last_name: String = get_input("Enter the last name: ");
    let zip_code: String = get_input("Enter the ZIP code: ");
    let employee_id: String = get_input("Enter an employee ID: ");

    let regex_not_empty: &str = r"!^\\s*$";
    let regex_min_2: &str = r"^.{2,}$";
    let regex_employee_id: &str = r"^[A-Z]{2}-\d{4}$";
    let regex_zip_code: &str = r"^\d+$";

    if let Err(_) = validate_regex(&first_name, regex_not_empty) {
        println!("The first name must be filled in.");
    }
    if let Err(_) = validate_regex(&last_name, regex_not_empty) {
        println!("The last name must be filled in.");
    }
    if let Err(_) = validate_regex(&first_name, regex_min_2) {
        println!("The first name must be at least two characters long.");
    }
    if let Err(_) = validate_regex(&last_name, regex_min_2) {
        println!("The last name must be at least two characters long.");
    }
    if let Err(_) = validate_regex(&employee_id, regex_employee_id) {
        println!("{} is not a valid ID.", employee_id);
    }
    if let Err(_) = validate_regex(&zip_code, regex_zip_code) {
        println!("The ZIP code must be numeric.");
    }
}
