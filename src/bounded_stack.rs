/// This data structure provides operations on a stack with a fixed capacity such that push is O(n), pop is O(1).
/// The inefficient O(n) push is acceptable since n is small, and since this vector implementation allocates memory
/// only once, it is much preferred over a linked list implementation.
#[derive(Debug)]
pub struct BoundedStack<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> BoundedStack<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        Self {
            items: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// If capacity would be exceeded, first delete the oldest item to make room,
    /// and then push the new item on top of stack.
    pub fn push(&mut self, item: T) {
        if self.items.len() == self.capacity {
            self.items.remove(0);
        }

        self.items.push(item);
    }

    /// Remove and get the newest item on top of stack.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Empty the stack.
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = BoundedStack::new(3);

        stack.push(0);
        assert_eq!(stack.items, vec![0]);

        stack.push(1);
        assert_eq!(stack.items, vec![0, 1]);

        stack.push(2);
        assert_eq!(stack.items, vec![0, 1, 2]);

        stack.push(3);
        assert_eq!(stack.items, vec![1, 2, 3]);

        stack.push(4);
        assert_eq!(stack.items, vec![2, 3, 4]);
    }

    #[test]
    fn test_pop() {
        let mut stack = BoundedStack::new(3);

        for i in 0..5 {
            stack.push(i);
        }
        assert_eq!(stack.items, vec![2, 3, 4]);

        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_clear() {
        let mut stack = BoundedStack::new(3);

        for i in 0..5 {
            stack.push(i);
        }
        assert_eq!(stack.items, vec![2, 3, 4]);

        stack.clear();
        assert_eq!(stack.items, vec![]);
    }

    #[test]
    #[should_panic]
    fn test_zero_capacity_should_panic() {
        let _: BoundedStack<u32> = BoundedStack::new(0);
    }
}
