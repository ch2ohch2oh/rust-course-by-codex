/*
Exercise 10A: Rust Attributes

Tasks:
1. Add one more derived trait to `BuildMode` and explain why it is appropriate.
2. Add a targeted lint attribute to one helper and justify it in a comment.
3. Add a second test under `#[cfg(test)]`.
4. Change or extend one attribute and observe how it affects compilation or behavior.
*/

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
enum BuildMode {
    Debug = 1,
    Release = 2,
}

// This helper is intentionally parked for a later lesson, so suppressing
// `dead_code` locally is more precise than weakening the crate-wide lint.
#[allow(dead_code)]
fn future_helper() -> &'static str {
    "reserved for later exercises"
}

#[must_use]
fn build_mode_name(mode: BuildMode) -> &'static str {
    match mode {
        BuildMode::Debug => "debug",
        BuildMode::Release => "release",
    }
}

fn main() {
    println!(
        "{:?} => {}",
        BuildMode::Debug,
        build_mode_name(BuildMode::Debug)
    );
    println!(
        "{:?} => {}",
        BuildMode::Release,
        build_mode_name(BuildMode::Release)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_partial_eq_works() {
        assert_eq!(BuildMode::Debug, BuildMode::Debug);
        assert_ne!(BuildMode::Debug, BuildMode::Release);
    }

    #[test]
    fn repr_u8_gives_explicit_discriminants() {
        assert_eq!(BuildMode::Debug as u8, 1);
        assert_eq!(BuildMode::Release as u8, 2);
    }
}
