// Ring buffer stolen from
// https://crates.io/crates/e-ring/0.3.0
// altered for my own stuff

/// Append only data structure, replace oldest element when reach maximum capacity of `N` elements
#[derive(Debug, Clone)]
pub struct Ring<T, const N: usize> {
    data: [T; N],
    head: usize,
    tail: usize,
    len: usize,
}

/// Iterator over `Ring` starting from the oldest element
impl<T: Copy + Default, const N: usize> Default for Ring<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Default, const N: usize> Ring<T, N> {
    /// Creates a new `Ring` of give size `N`
    pub fn new() -> Self {
        Ring {
            data: [T::default(); N],
            head: 0usize,
            tail: 0usize,
            len: 0usize,
        }
    }

    fn increment_tail(&mut self) {
        self.tail = (self.tail + 1) % self.data.len()
    }

    fn increment_head(&mut self) {
        self.head = (self.head + 1) % self.data.len()
    }

    /// Append an element to the `Ring`, if there are already `N` elements, it replaces the oldest.
    pub fn append(&mut self, el: T) {
        self.data[self.tail] = el;
        self.increment_tail()
    }

    /// Number of elements in the `Ring`, it never decreases.
    pub fn len(&self) -> usize {
        self.len
    }

    /// If the `Ring` is empty. Zero elements
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /// Return the max size of the ring
    pub fn size(&self) -> usize {
        N
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let result = self.data[self.head].clone();
        self.increment_head();
        Some(result)
    }
}
