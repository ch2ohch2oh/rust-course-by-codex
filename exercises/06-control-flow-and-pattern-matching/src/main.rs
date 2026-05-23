/*
Exercise 06: Control Flow and Pattern Matching

Tasks:
1. Extend the enum with one more variant.
2. Add a `match` arm for every variant.
3. Build a tiny guessing or menu loop with `loop`, `if`, and `match`.
4. Add one `while let` example.
*/

enum Command {
    Start,
    Pause,
    Stop,
}

fn command_name(command: Command) -> &'static str {
    match command {
        Command::Start => "starting",
        Command::Pause => "pausing",
        Command::Stop => "stopping",
    }
}

fn main() {
    for turn in 1..=3 {
        if turn % 2 == 0 {
            println!("turn {turn}: even");
        } else {
            println!("turn {turn}: odd");
        }
    }

    println!("{}", command_name(Command::Start));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_command_has_a_name() {
        assert_eq!(command_name(Command::Start), "starting");
        assert_eq!(command_name(Command::Pause), "pausing");
        assert_eq!(command_name(Command::Stop), "stopping");
    }
}
