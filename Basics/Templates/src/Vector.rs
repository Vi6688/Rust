

pub struct Vector<T> {
    pub element: Vec<T>,
}
impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector { element: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.element.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.element.pop()
    }

    pub fn len(&self) -> usize {
        self.element.len()
    }

    pub fn is_empty(&self) -> bool {
        self.element.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.element.get(index)
    }
    pub fn push(&mut self, value: T) {
        self.element.push(value);
    }
}