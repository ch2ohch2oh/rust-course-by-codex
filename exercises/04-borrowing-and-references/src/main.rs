/*
Exercise 04: Borrowing and References

Tasks:
1. Use `measure` to read a string by reference.
2. Use `append_rust` to mutate a string through `&mut String`.
3. Intentionally create a borrow checker error, read it carefully, then fix it.
4. Rewrite one ownership-heavy example from the previous exercise using borrowing.
*/

fn measure(text: &str) -> usize {
    text.len()
}

fn append_rust(text: &mut String) {
    text.push_str(" Rust");
}

fn main() {
    let mut label = String::from("Learning");
    
    // TODO: Call `measure` on `label` to get its length
    let len = measure(&label);

    // TODO: Call `append_rust` on `label` to mutate it
    append_rust(&mut label);

    // TODO: Print the modified label and its original length
    println!("{label} has length {len} before mutation");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_reads_without_taking_ownership() {
        let value = String::from("borrow me");
        assert_eq!(measure(&value), 9);
        assert_eq!(value, "borrow me");
    }

    #[test]
    fn append_rust_mutates_in_place() {
        let mut value = String::from("Learning");
        append_rust(&mut value);
        assert_eq!(value, "Learning Rust");
    }
}
