/*
Exercise 02: Variables, Mutability, and Basic Types

Tasks:
1. Finish `celsius_to_fahrenheit`.
2. Use a tuple to store a name and age, then print both fields.
3. Sum the values in `scores`.
4. Demonstrate the difference between shadowing and mutation.
*/

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn main() {
    let mut language = String::from("Python");
    language.push_str(" and Rust");

    let spaces = "   ";
    let spaces = spaces.len();

    let person = ("Avery", 31);
    let scores = [10, 20, 30, 40, 50];

    println!("{} is {} years old", person.0, person.1);
    println!("32C = {}F", celsius_to_fahrenheit(32.0));
    println!("Language: {language}");
    println!("Whitespace count: {spaces}");

    // TODO: Print the sum of `scores`.
    let _ = scores;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }
}
