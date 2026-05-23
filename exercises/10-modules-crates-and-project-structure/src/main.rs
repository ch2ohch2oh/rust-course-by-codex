/*
Exercise 10: Modules, Crates, and Project Structure

Tasks:
1. Split this file into multiple modules.
2. Move reusable logic into `src/lib.rs`.
3. Keep only the public API public.
4. Make `main.rs` a thin layer that calls into library code.
*/

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("6 * 7 = {}", multiply(6, 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_returns_expected_result() {
        assert_eq!(multiply(6, 7), 42);
    }
}
