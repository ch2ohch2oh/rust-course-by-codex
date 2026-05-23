/*
Exercise 03: Ownership and Moves

Tasks:
1. Add comments describing who owns each value after every major line.
2. Create one example where a `String` is moved into a function.
3. Create a second example using `.clone()`.
4. Compare that with `i32`, which is copied.
*/

fn takes_string(value: String) {
    // Rust 1.58+ allows capturing variables directly inside the formatting string {}
    println!("owned string: {value}");
}

fn takes_integer(value: i32) {
    println!("copied integer: {value}");
}

fn clone_and_return_text(value: &str) -> String {
    value.to_owned()
}

fn copy_and_return_number(value: i32) -> i32 {
    value
}

fn main() {
    let greeting = String::from("hello");
    let number = 42;

    let cloned_greeting = clone_and_return_text(&greeting);
    takes_string(cloned_greeting);
    println!("greeting is still usable after cloning: {greeting}");

    let moved_greeting = greeting;
    takes_string(moved_greeting);

    let copied_number = copy_and_return_number(number);
    takes_integer(copied_number);
    takes_integer(number);
    println!("number is still usable here: {number}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cloning_keeps_original_value_usable() {
        let text = String::from("hello");
        let cloned = clone_and_return_text(&text);

        assert_eq!(text, "hello");
        assert_eq!(cloned, "hello");
    }

    #[test]
    fn integers_are_copied() {
        let number = 42;
        let copied = copy_and_return_number(number);

        assert_eq!(number, 42);
        assert_eq!(copied, 42);
    }
}
