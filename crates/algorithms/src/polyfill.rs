//! Implementation of traits that are not yet available in error-stack and friends, but are
//! tremendously useful.

use error_stack::Result;

trait Container<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;

    fn extend_one(&mut self, item: T);
}

impl<T> Container<T> for Vec<T> {
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn extend_one(&mut self, item: T) {
        self.push(item);
    }
}

pub(crate) trait IteratorExt {
    type Item;
    type Context;

    fn collect_reports<T>(self) -> Result<T, Self::Context>
    where
        T: Container<Self::Item>;
}

impl<I, T, C> IteratorExt for I
where
    I: Iterator<Item = Result<T, C>>,
{
    type Context = C;
    type Item = T;

    fn collect_reports<F>(self) -> Result<F, Self::Context>
    where
        F: Container<Self::Item>,
    {
        let (_, max) = self.size_hint();

        let state = if let Some(max) = max {
            F::with_capacity(max)
        } else {
            F::new()
        };

        let mut state: Result<F, Self::Context> = Ok(state);

        for item in self {
            match (&mut state, item) {
                (Err(state), Err(error)) => {
                    state.extend_one(error);
                }
                (Err(_), Ok(_)) => {}
                (state @ Ok(_), Err(error)) => {
                    *state = Err(error);
                }
                (Ok(state), Ok(item)) => {
                    state.extend_one(item);
                }
            }
        }

        state
    }
}