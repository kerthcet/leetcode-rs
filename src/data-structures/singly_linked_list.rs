// This is a LIFO list.
struct SinglyLinkedList<T> {
    head: Box<Node<T>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Default> SinglyLinkedList<T> {

    fn new() -> Self {
        Self {
            // Initialize a head node.
            head: Box::new(Node{
                value: T::default(),
                next: None,
            }),
        }
    }

    fn push(&mut self, element: T) {
        let new_node = Box::new(Node{
            value: element,
            next: self.head.next.take(),
        });
        self.head.next = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.next.take().map(|node| {
            self.head.next = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.next.as_ref().map(|node| &node.value )
    }

}

fn main() {
    let mut list: SinglyLinkedList<i8> = SinglyLinkedList::new();
    let mut res = list.pop();
    assert_eq!(res, None, "It should be None");

    list.push(1);
    let peek_res = list.peek();
    assert_eq!(peek_res, Some(1).as_ref(), "It should be 1");
    res = list.pop();
    assert_eq!(res, Some(1), "It should be 1");
    res = list.pop();
    assert_eq!(res, None, "It should be None");

    list.push(2);
    list.push(3);
    list.push(4);
    res = list.pop();
    assert_eq!(res, Some(4), "It should be 4");
    res = list.pop();
    assert_eq!(res, Some(3), "It should be 3");
    res = list.pop();
    assert_eq!(res, Some(2), "It should be 2");

    println!("Succeeded!")
}