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
        modules_crates_and_project_structure::multiply(6, 7)
    );
}
