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
    // `Box<T>` gives this recursive enum a known size at compile time.
    // It also means the list has one clear owner.
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    // `Rc<T>` stands for `Reference Counted<T>`.
    // It lets multiple values own the same data on one thread.
    // By itself, it is for shared read access rather than direct mutation.
    let shared = Rc::new(String::from("shared data"));
    let _a = Rc::clone(&shared);
    let _b = Rc::clone(&shared);

    // `RefCell<T>` allows interior mutability.
    // The outer binding does not need to be `mut`, but borrow rules are checked at runtime.
    // If you need shared ownership plus mutation on one thread, `Rc<RefCell<T>>` is a common pattern.
    // `Arc<T>` is the multi-threaded version of `Rc<T>` and is often paired with `Mutex<T>`.
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
