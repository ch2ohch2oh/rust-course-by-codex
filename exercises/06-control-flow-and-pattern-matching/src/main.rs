/*
Exercise 06: Control Flow and Pattern Matching

Tasks:
1. Extend the enum with one more variant.
2. Add a `match` arm for every variant.
3. Build a tiny guessing or menu loop with `loop`, `if`, and `match`.
4. Add one `while let` example.
*/

// This enum is small and has no owned data, so Clone + Copy are a natural fit.
#[derive(Clone, Copy)]
enum Command {
    Start,
    Pause,
    Resume,
    Stop,
    Unknown
}

// String literals live for the whole program, so &'static str works here,
// though writing just &str would also be fine.
fn command_name(command: Command) -> &'static str {
    match command {
        Command::Start => "starting",
        Command::Pause => "pausing",
        Command::Resume => "resuming",
        Command::Stop => "stopping",
        Command::Unknown => "unknown",
    }
}

fn main() {
    let commands = [
        Command::Start,
        Command::Pause,
        Command::Resume,
        Command::Stop,
    ];

    for turn in 1..=3 {
        if turn % 2 == 0 {
            println!("turn {turn}: even");
        } else {
            println!("turn {turn}: odd");
        }
    }

    for command in commands {
        println!("{}", command_name(command));
    }

    let mut menu_step = 0;
    loop {
        menu_step += 1;
        match menu_step {
            1 => println!("menu: start"),
            2 => println!("menu: pause"),
            _ => {
                println!("menu: stop");
                break;
            }
        }
    }

    let mut remaining = Some(3_u32);
    while let Some(value) = remaining {
        println!("countdown: {value}");
        remaining = value.checked_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_command_has_a_name() {
        assert_eq!(command_name(Command::Start), "starting");
        assert_eq!(command_name(Command::Pause), "pausing");
        assert_eq!(command_name(Command::Resume), "resuming");
        assert_eq!(command_name(Command::Stop), "stopping");
    }
}
