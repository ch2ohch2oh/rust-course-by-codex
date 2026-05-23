/*
Exercise 09: Error Handling

Tasks:
1. Replace the `unwrap_or` shortcut with explicit `Result` handling.
2. Add a function that returns `Option<i32>` for the first parsed number.
3. Change `main` to return `Result<(), Box<dyn std::error::Error>>`.
4. Add a file-reading example that uses `?`.
*/

fn parse_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    text.parse::<i32>()
}

fn first_parsed_number(values: &[&str]) -> Option<i32> {
    values.iter().find_map(|value| parse_number(value).ok())
}

fn main() {
    let values = ["10", "20", "oops"];

    for value in values {
        let parsed = parse_number(value).unwrap_or(-1);
        println!("{value} -> {parsed}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_succeeds_for_valid_input() {
        assert_eq!(parse_number("42").unwrap(), 42);
    }

    #[test]
    fn first_parsed_number_skips_invalid_values() {
        assert_eq!(first_parsed_number(&["oops", "11", "12"]), Some(11));
        assert_eq!(first_parsed_number(&["bad", "worse"]), None);
    }
}
