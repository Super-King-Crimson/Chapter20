pub trait Tap<F, T=Self> where F: FnOnce(&T) -> () {
    fn tap(self, f: F) -> T;
}

impl<F, T, E> Tap<F> for Result<T, E> where F: FnOnce(&Self) {
    fn tap(self, f: F) -> Self {
        f(&self);
        self
    }
}

pub trait TapMut<F, T=Self> where F: FnOnce(&mut T) {
    fn tap_mut(self, f: F) -> T;
}

impl<F, T, E> TapMut<F> for Result<T, E> where F: FnOnce(&mut Self) {
    fn tap_mut(mut self, f: F) -> Self {
        f(&mut self);
        self
    }
}