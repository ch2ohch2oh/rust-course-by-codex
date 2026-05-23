/*
Exercise 14: Closures and Functional Style

Tasks:
1. Sort `users` by score.
2. Add a closure that captures and mutates local state.
3. Parse numeric strings and discard invalid entries.
4. Rewrite one loop into an iterator pipeline.
*/

#[derive(Debug)]
struct User {
    name: String,
    score: u32,
}

fn parse_numbers(words: &[&str]) -> Vec<i32> {
    words.iter().filter_map(|w| w.parse().ok()).collect()
}

fn count_long_words(words: &[&str], min_len: usize) -> usize {
    let mut count = 0;
    let mut record = |word: &&str| {
        if word.len() >= min_len {
            count += 1;
        }
    };

    for word in words {
        record(word);
    }

    count
}

fn main() {
    let mut users = vec![
        User {
            name: String::from("A"),
            score: 30,
        },
        User {
            name: String::from("B"),
            score: 10,
        },
    ];

    users.sort_by_key(|user| user.score);
    println!("{users:?}");

    let words = ["1", "2", "bad", "4"];
    let parsed = parse_numbers(&words);
    println!("{parsed:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_discards_invalid_entries() {
        assert_eq!(parse_numbers(&["1", "x", "3"]), vec![1, 3]);
    }

    #[test]
    fn sorting_by_score_places_lowest_first() {
        let mut users = vec![
            User {
                name: String::from("A"),
                score: 30,
            },
            User {
                name: String::from("B"),
                score: 10,
            },
        ];
        users.sort_by_key(|user| user.score);
        assert_eq!(users[0].name, "B");
        assert_eq!(users[1].name, "A");
    }

    #[test]
    fn closure_can_capture_and_mutate_state() {
        assert_eq!(count_long_words(&["rust", "is", "fun"], 3), 2);
    }
}
