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

fn validate_name(name: &str) -> Result<(), &str> {
    if name.is_empty() {
        return Err("The {} must be filled in.");
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
