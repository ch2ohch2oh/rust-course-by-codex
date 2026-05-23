/*
Exercise 18: Unsafe Rust and FFI

Tasks:
1. Explain why the raw pointer dereference below is safe.
2. Keep raw-pointer dereferences behind an `unsafe fn` or behind a truly safe wrapper that proves validity.
3. Add a short safety comment documenting the invariant.
4. Optionally extend this into a tiny FFI example with `extern "C"`.
*/

unsafe fn read_valid_ptr(ptr: *const i32) -> i32 {
    // SAFETY: callers must pass a non-null, properly aligned pointer to a live `i32`.
    *ptr
}

fn main() {
    let value = 123;
    let ptr = &value as *const i32;

    let dereferenced = unsafe { read_valid_ptr(ptr) };

    println!("dereferenced = {dereferenced}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_wrapper_reads_pointer_value() {
        let value = 99;
        let ptr = &value as *const i32;
        let result = unsafe { read_valid_ptr(ptr) };
        assert_eq!(result, 99);
    }
}
