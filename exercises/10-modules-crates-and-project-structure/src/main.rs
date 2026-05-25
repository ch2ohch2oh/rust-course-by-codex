/*
Exercise 10: Modules, Crates, and Project Structure

Tasks:
1. Split this file into multiple modules.
2. Move reusable logic into `src/lib.rs`.
3. Keep only the public API public.
4. Make `main.rs` a thin layer that calls into library code.
*/

fn main() {
    println!(
        "6 * 7 = {}",
        // This package has a binary crate (`main.rs`) and a library crate (`lib.rs`).
        // The binary calls the library through the crate name from `Cargo.toml`.
        modules_crates_and_project_structure::multiply(6, 7)
    );
}
