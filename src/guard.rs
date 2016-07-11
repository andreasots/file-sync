use std::ops::Deref;

pub struct Guard<T, F: FnOnce(T)> {
    data: Option<(T, F)>,
}

impl<T, F: FnOnce(T)> Guard<T, F> {
    pub fn new(value: T, dtor: F) -> Guard<T, F> {
        Guard {
            data: Some((value, dtor))
        }
    }
}

impl<T, F: FnOnce(T)> Deref for Guard<T, F> {
    type Target = T;

    fn deref(&self) -> &T {
        match self.data {
            Some((ref value, _)) => value,
            None => unreachable!(),
        }
    }
}

impl<T, F: FnOnce(T)> Drop for Guard<T, F> {
    fn drop(&mut self) {
        match self.data.take() {
            Some((value, dtor)) => dtor(value),
            None => unreachable!(),
        }
    }
}
