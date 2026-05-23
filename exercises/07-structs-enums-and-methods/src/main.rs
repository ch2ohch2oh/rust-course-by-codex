/*
Exercise 07: Structs, Enums, and Methods

Tasks:
1. Add an `area` method to `Rectangle`.
2. Add a `can_hold` method.
3. Add one more `Message` variant carrying data.
4. Handle every `Message` variant in `process`.
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

enum Message {
    Quit,
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn process(message: Message) {
    match message {
        Message::Quit => println!("quit"),
        Message::Write(text) => println!("text: {text}"),
        Message::ChangeColor(red, green, blue) => {
            println!("color: rgb({red}, {green}, {blue})")
        }
    }
}

fn main() {
    let rect = Rectangle::square(4);
    let shelf = Rectangle {
        width: 8,
        height: 8,
    };

    println!("{rect:?} area={}", rect.area());
    println!("shelf can hold rect? {}", shelf.can_hold(&rect));
    process(Message::Quit);
    process(Message::Write(String::from("hello")));
    process(Message::ChangeColor(20, 40, 60));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_area_is_correct() {
        let rect = Rectangle::square(4);
        assert_eq!(rect.area(), 16);
    }

    #[test]
    fn can_hold_checks_both_dimensions() {
        let outer = Rectangle {
            width: 10,
            height: 8,
        };
        let inner = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(outer.can_hold(&inner));
        assert!(!inner.can_hold(&outer));
    }
}
