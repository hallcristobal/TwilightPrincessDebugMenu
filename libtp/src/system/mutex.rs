use core::cell::RefCell;
use core::ops::Deref;

// Bad should us actual mutex
pub struct Mutex<T>(pub RefCell<T>);
unsafe impl<T> Sync for Mutex<T> {}
impl<T> Deref for Mutex<T> {
    type Target = RefCell<T>;
    fn deref(&self) -> &RefCell<T> {
        &self.0
    }
}
