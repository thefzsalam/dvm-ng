use std::cell::RefCell;
use std::ops::Deref;

/**
 * A container implementing `Deref` trait.
 * A less cumbersome alternative to `Option<T>`, for special cases.
 * It is used by `MainApp` to overcome circular dependency of `struct`s.
 * Make sure that `initialize(value: T)` is called as soon as possible.
 * Dereferencing before calling `initialize` will panic.
 */
pub struct DelayedInitializer<T>(Option<T>);

impl<T> DelayedInitializer<T> {
    pub fn new_uninitialized() -> DelayedInitializer<T> {
        DelayedInitializer(Option::None)
    }

    pub fn initialize(&mut self, value: T) {
        self.0 = Option::Some(value);
    }
}

impl<T> Deref for DelayedInitializer<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.as_ref().expect("DelayedInitialzer not initialized.")
    }
}