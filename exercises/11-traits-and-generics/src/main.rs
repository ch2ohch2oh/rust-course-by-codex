/*
Exercise 11: Traits and Generics

Tasks:
1. Finish `largest`.
2. Implement `Describe` for at least two types.
3. Add a generic struct such as `Point<T>`.
4. Write a function that accepts any `Describe`.
*/

trait Describe {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
}

struct Movie {
    title: String,
}

struct Point<T> {
    x: T,
    y: T,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("Book: {}", self.title)
    }
}

impl Describe for Movie {
    fn describe(&self) -> String {
        format!("Movie: {}", self.title)
    }
}

fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut current = items.first()?;
    for item in &items[1..] {
        if item > current {
            current = item;
        }
    }
    Some(current)
}

fn print_description(item: &impl Describe) -> String {
    item.describe()
}

fn main() {
    let values = [3, 8, 5, 2];
    let book = Book {
        title: String::from("Rust Patterns"),
    };
    let movie = Movie {
        title: String::from("Ferris Bueller"),
    };
    let point = Point { x: 3, y: 4 };

    println!("largest = {:?}", largest(&values));
    println!("{}", print_description(&book));
    println!("{}", print_description(&movie));
    println!("point = ({}, {})", point.x, point.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_finds_maximum() {
        assert_eq!(largest(&[3, 8, 5, 2]), Some(&8));
        assert_eq!(largest::<i32>(&[]), None);
    }

    #[test]
    fn describe_implementation_is_readable() {
        let book = Book {
            title: String::from("Rust Patterns"),
        };

        assert_eq!(book.describe(), "Book: Rust Patterns");
    }

    #[test]
    fn print_description_accepts_any_describe() {
        let movie = Movie {
            title: String::from("Ferris Bueller"),
        };

        assert_eq!(print_description(&movie), "Movie: Ferris Bueller");
    }

    #[test]
    fn generic_point_holds_coordinates() {
        let point = Point { x: 1.5, y: 2.5 };
        assert_eq!(point.x, 1.5);
        assert_eq!(point.y, 2.5);
    }
}
