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

fn main() {
    let text = String::from("hello rust world");
    let csv = "red,green,blue";

    greet(&text);
    println!("first word = {}", first_word(&text));

    // TODO: Split `csv` into fields and print them.
    let _ = csv;
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
}
