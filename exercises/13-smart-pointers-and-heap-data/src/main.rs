/*
Exercise 13: Smart Pointers and Heap Data

Tasks:
1. Extend the recursive list example.
2. Add an `Rc<String>` example with two owners.
3. Add a `RefCell<i32>` mutation example.
4. Write comments describing when you would choose `Box`, `Rc`, or `Arc`.
*/

use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn sum_list(list: &List) -> i32 {
    match list {
        List::Cons(value, next) => value + sum_list(next),
        List::Nil => 0,
    }
}

fn main() {
    // `Box` gives recursive data a known size at compile time.
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    // `Rc` shares read-only ownership on a single thread.
    let shared = Rc::new(String::from("shared data"));
    let _a = Rc::clone(&shared);
    let _b = Rc::clone(&shared);

    // `Arc` is the thread-safe sharing variant; pair it with `Mutex` when writes are needed.
    let value = RefCell::new(10);
    *value.borrow_mut() += 5;

    println!("list sum = {}", sum_list(&list));
    println!("shared = {shared}");
    println!("value = {}", value.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_list_can_be_summed() {
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
        assert_eq!(sum_list(&list), 6);
    }

    #[test]
    fn rc_tracks_shared_owners() {
        let shared = Rc::new(String::from("shared"));
        let _clone = Rc::clone(&shared);
        assert_eq!(Rc::strong_count(&shared), 2);
    }
}
