/*
Exercise 05: Strings and Slices

Tasks:
1. Finish `first_word`.
2. Write another function that accepts `&str`, not `String`.
3. Split a comma-separated line into fields.
4. Explain in comments why direct indexing into a `String` is restricted.
*/

fn first_word(s: &str) -> &str {
    for (i, b) in s.as_bytes().iter().enumerate() {
        if *b == b' ' {
            return &s[..i];
        }
    }

    s
}

fn greet(name: &str) {
    println!("Hello, {name}");
}

fn split_csv_line(line: &str) -> Vec<&str> {
    // `line` is `&str`, `line.split(',')` returns `std::str::Split<'_, char>`,
    // where `'_` is the inferred lifetime saying the iterator borrows from `line`.
    // Each item yielded by that iterator is `&str`, and `.collect()` gathers them
    // into the final `Vec<&str>`.
    line.split(',').collect()
}

fn main() {
    let text = String::from("hello rust world");
    let csv = "red,green,blue";

    greet(&text);
    println!("first word = {}", first_word(&text));

    // Direct indexing into `String` is restricted because strings are UTF-8.
    // A single character may span multiple bytes, so `s[0]` would be ambiguous.
    let fields = split_csv_line(csv);
    println!("fields = {fields:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_returns_prefix_before_space() {
        assert_eq!(first_word("hello rust"), "hello");
    }

    #[test]
    fn first_word_returns_whole_input_when_no_space_exists() {
        assert_eq!(first_word("rust"), "rust");
    }

    #[test]
    fn split_csv_line_breaks_fields_apart() {
        assert_eq!(
            split_csv_line("red,green,blue"),
            vec!["red", "green", "blue"]
        );
    }
}
