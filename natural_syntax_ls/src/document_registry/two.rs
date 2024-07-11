use super::*;

#[derive(Debug)]
pub(crate) struct Two<T> {
    first: Option<T>,
    second: Option<T>,
}

impl<T> Two<T> {
    /// Take out the older item and guarantee the newer one remains.
    pub(crate) fn take_older(&mut self) -> Option<T> {
        match (&mut self.first, &mut self.second) {
            (first @ Some(_), second @ Some(_)) => {
                mem::swap(first, second);
                mem::take(second)
            }
            _ => None,
        }
    }

    /// Take out the newer item and clear both slots.
    pub(crate) fn take_newer_n_clear(&mut self) -> Option<T> {
        match (mem::take(&mut self.first), mem::take(&mut self.second)) {
            (first @ Some(_), None) => first,
            (_assumed_some, second @ Some(_)) => second,
            _ => None,
        }
    }

    /// Append a new item, returning the older item if there already are 2.
    pub(crate) fn push(&mut self, t: T) -> Option<T> {
        match (&mut self.first, &mut self.second) {
            (first @ Some(_), second @ Some(_)) => {
                mem::swap(first, second);
                second.replace(t)
            }
            (_assumed_some, second @ None) => second.replace(t),
            (first @ None, _assumed_none) => first.replace(t),
        }
    }
}

impl<T> Default for Two<T> {
    fn default() -> Self {
        Self {
            first: None,
            second: None,
        }
    }
}
