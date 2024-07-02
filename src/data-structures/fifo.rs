// Queue is a FIFO queue.
#[derive(Debug)]
struct FIFO<T> {
    // indicates the first item's index.
    start: usize,
    // indicates the last item's index+1.
    end: usize,
    // indicates the items' length.
    len: usize,
    // With a default capacity of 4.
    elements: Vec<T>,
}

impl<T: Default + Copy> FIFO<T> {

    fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            len: 0,
            elements: vec!(T::default(); 4),
        }
    }

    fn enqueue(&mut self, item: T) {
        let size = self.len;
        if size == self.elements.capacity() {
            // Scale 2x size once reach the limit of capacity.
            let mut new_elements = vec!(T::default(); size * 2);
            if self.start == 0 {
                new_elements[..size].copy_from_slice(&self.elements);
            } else {
                new_elements[..size-self.start].copy_from_slice(&self.elements[self.start..size]);
                new_elements[(size-self.start)..size].copy_from_slice(&self.elements[0..self.end]);
                self.start = 0;
                self.end = size;
            }
            self.elements = new_elements;
        }

        if self.end == self.elements.capacity() {
            self.end = 0;
        }

        self.elements[self.end] = item;
        self.end += 1;
        self.len += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.start == self.end {
            return None
        }

        let ret: T = self.elements[self.start];
        self.start += 1;
        self.len -=1;

        if self.start == self.elements.capacity() {
            self.start = 0
        }

        // Reset the queue once queue is empty.
        if self.start == self.end {
            self.elements = vec!(T::default(); 4);
            self.start = 0;
            self.end = 0;
        }

        return Some(ret);
    }

    fn is_empty(&self) -> bool {
        return self.len == 0
    }

    fn len(&self) -> usize {
        return self.len
    }

    fn capacity(&self) -> usize {
        return self.elements.capacity()
    }

}

fn main() {
    let mut queue: FIFO<i32> = FIFO::new();
    assert!(queue.is_empty(), "queue should be empty");
    assert_eq!(queue.len(), 0, "queue length should be 0");
    assert_eq!(queue.capacity(), 4, "queue capacity should be default 4");

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.enqueue(4);
    assert_eq!(queue.len(), 4, "queue length should be 4");
    assert_eq!(queue.capacity(), 4, "queue capacity should be default 4");

    let mut res = queue.dequeue();
    assert_eq!(res, Some(1), "value should be 1");
    res = queue.dequeue();
    assert_eq!(res, Some(2), "value should be 2");

    queue.enqueue(5);
    queue.enqueue(6);
    assert_eq!(queue.len(), 4, "queue length should be 4");
    assert_eq!(queue.capacity(), 4, "queue capacity should be default 4");

    queue.enqueue(7);
    assert_eq!(queue.len(), 5, "queue length should be 5");
    assert_eq!(queue.capacity(), 8, "queue capacity should be default 8");

    res = queue.dequeue();
    assert_eq!(res, Some(3), "value should be 3");
    res = queue.dequeue();
    assert_eq!(res, Some(4), "value should be 4");
    res = queue.dequeue();
    assert_eq!(res, Some(5), "value should be 5");
    res = queue.dequeue();
    assert_eq!(res, Some(6), "value should be 6");
    res = queue.dequeue();
    assert_eq!(res, Some(7), "value should be 7");
    res = queue.dequeue();
    assert_eq!(res, None, "value should be None");
    assert_eq!(queue.len(), 0, "queue length should be 0");
    assert_eq!(queue.capacity(), 4, "queue capacity should be default 4");

    println!("Succeed!")
}
