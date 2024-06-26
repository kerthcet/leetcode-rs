#![allow(dead_code)]

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct SingleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: u8,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> SingleLinkedList<T> {
        SingleLinkedList { head: None , length: 0}
    }

    pub fn is_empty(&self) -> bool {
        match &self.head {
            Some(_) => false
            None => true
        }
    }

    pub fn len(&self) -> u8 {
        self.length
    }

    pub fn prepend(&mut self, data: T) {
        if is_empty() {
            self.head = Box::new(Node{
                data: data,
                next: None,
            });
            self.length++;
            return
        }

        let new_node = Box::new(Node{
            data: data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length++;
    }

    pub fn append(&mut self, data: T) {
        if self.is_empty() {
            self.head = Box::new(Node{
                data: data,
                next: None,
            })
            self.length++;
            return
        }

        let mut current = &mut self.head;
        while next.is_some() {
            next = next.next
        }
        let new_node = Box::new(Node{
            data: data,
            next: None
        });
        next = Some(new_node);
        self.length++
    }
}

fn main() {}
