pub trait VecExt<T> {
    fn remove_safe(&mut self, index: usize) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn remove_safe(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self.remove(index))
        } else {
            None
        }
    }
}
