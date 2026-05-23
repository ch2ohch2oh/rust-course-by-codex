/*
Exercise 08: Collections and Iterators

Tasks:
1. Compute the sum and average of `numbers`.
2. Filter even numbers into a new vector.
3. Count word frequencies in `text`.
4. Rewrite one manual loop as an iterator chain.
*/

use std::collections::HashMap;

fn even_numbers(values: &[i32]) -> Vec<i32> {
    values.iter().copied().filter(|n| n % 2 == 0).collect()
}

fn word_count(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0usize) += 1;
    }
    counts
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let text = "rust makes ownership explicit and rust makes safety practical";

    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    println!("doubled = {doubled:?}");

    let counts = word_count(text);
    println!("counts = {counts:?}");

    // TODO: Compute sum and average for `numbers`.
    // TODO: Collect the even values into a new vector.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_even_numbers() {
        assert_eq!(even_numbers(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    }

    #[test]
    fn counts_words() {
        let counts = word_count("rust rust python");
        assert_eq!(counts.get("rust"), Some(&2));
        assert_eq!(counts.get("python"), Some(&1));
    }
}
