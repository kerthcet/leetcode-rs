use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

// You can not use Box directly here because it may have several references.
struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T: Default> DoublyLinkedList<T> {

    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn push_front(&mut self, element: T) {
        let new_node = Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            value: element,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
        self.len += 1;
    }

    fn push_back(&mut self, element: T) {
        let new_node = Rc::new(RefCell::new(Node{
            prev: None,
            next: None,
            value: element,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.len += 1;
    }

    fn pop_front(&mut self) ->  Option<T> {
        self.head.take().map(|head| {
            match head.borrow_mut().next.take() {
                Some(node) => {
                    node.borrow_mut().prev.take();
                    self.head = Some(node);
                }
                None => {
                    self.tail.take();
                }
            }
            self.len -= 1;
            Rc::try_unwrap(head).ok().unwrap().into_inner().value
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            match tail.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next.take();
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            self.len -= 1;
            Rc::try_unwrap(tail).ok().unwrap().into_inner().value
        })
    }

    fn len(&self) -> usize {
        return self.len
    }

}

fn main() {
    let mut list: DoublyLinkedList<u8> = DoublyLinkedList::new();

    let mut res = list.pop_front();
    assert_eq!(res, None, "It should be None");
    assert_eq!(0, list.len(), "The length should be 0");

    list.push_front(1);
    list.push_front(2);
    assert_eq!(2, list.len(), "The length should be 2");

    res = list.pop_back();
    assert_eq!(res, Some(1), "Value should be 1");
    assert_eq!(1, list.len(), "The length should be 1");

    list.push_back(3);
    res = list.pop_front();
    assert_eq!(res, Some(2), "Value should be 2");
    assert_eq!(1, list.len(), "The length should be 1");

    res = list.pop_back();
    assert_eq!(res, Some(3), "Value should be 3");
    assert_eq!(0, list.len(), "The length should be 0");

    println!("Succeeded")
}
