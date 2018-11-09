use std::ptr;

struct myLinkedList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> myLinkedList<T> {
    fn new() -> Self {
        myLinkedList {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    fn push(&mut self, elem: T) {
        let mut new_head = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        let raw_head: *mut _ = &mut *new_head;

        if self.tail.is_null() {
            self.tail = raw_head;
        }

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            let node = *n;
            self.head = node.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            node.elem
        })
    }

    fn add(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }

        self.tail = raw_tail;
    }

    fn remove(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            let node = *n;
            self.head = node.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            node.elem
        })
    }
}

fn main() {}

mod test {
    use super::myLinkedList;

    #[test]
    fn basics() {
        let mut list = myLinkedList::new();

        list.push(1);
        list.add(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));

        list.add(4);

        assert_eq!(list.remove(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.remove(), Some(4));
        assert_eq!(list.pop(), None);
    }

}
