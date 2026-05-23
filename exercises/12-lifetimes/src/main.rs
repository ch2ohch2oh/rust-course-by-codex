/*
Exercise 12: Lifetimes

Tasks:
1. Explain what `'a` means in `longest`.
2. Add a struct that holds a string slice reference.
3. Write one example that is easier if the type owns a `String` instead.
4. Add comments describing the reference relationships.
*/

struct Quote<'a> {
    text: &'a str,
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn owned_emphasis(text: &str) -> String {
    format!("{text}!")
}

fn main() {
    let a = String::from("short");
    let b = String::from("a much longer string");
    let quote = Quote { text: &b };

    // `'a` ties the returned slice to the shorter of the two input lifetimes,
    // which prevents the result from outliving either borrowed input.
    println!("longest = {}", longest(&a, &b));
    println!("quote = {}", quote.text);
    println!("owned emphasis = {}", owned_emphasis(quote.text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_returns_left_when_lengths_match_or_left_is_longer() {
        assert_eq!(longest("abcd", "ab"), "abcd");
        assert_eq!(longest("same", "size"), "same");
    }

    #[test]
    fn longest_returns_right_when_right_is_longer() {
        assert_eq!(longest("hi", "there"), "there");
    }

    #[test]
    fn quote_holds_a_borrowed_slice() {
        let text = String::from("quoted");
        let quote = Quote { text: &text };
        assert_eq!(quote.text, "quoted");
    }
}
