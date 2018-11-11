use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;


// node to hold element and address..
struct Node<T> {
    item: T,
    next: Link<T>,
}
pub struct Queue<T> {
    first: Link<T>,
    last: Link<T>,
    length: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            first: None,
            last: None,
            length: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    pub fn size(&self) -> usize {
        self.length
    }
    pub fn enqueue(&mut self, item: T) {
        let temp = self.last.take();
        self.last = Some(Rc::new(RefCell::new(Node { item, next: None })));
        if self.is_empty() {
            self.first = self.last.clone();
        } else {
            let temp = temp.unwrap();
            temp.borrow_mut().next = self.last.clone();
        }
        self.length += 1;
    }

    pub fn dequeue(&mut self) -> Result<T, String> {

        let first = try!(self.first.take().ok_or("Queue is empty".to_owned()));

        if Rc::strong_count(&first) == 2 {
            self.last = None;
        }
        let first_node = Rc::try_unwrap(first).ok().expect(
        ).into_inner();
        self.first = first_node.next;
        self.length -= 1;
        Ok(first_node.item)
    }

    pub fn dequeue_back(&mut self) -> Result<T, String> {

        let last = try!(self.last.take().ok_or("Queue is empty".to_owned()));
        self.length -= 1;


        {
            let mut search = self.first.clone().expect(


            let potential_next = search.borrow().next.clone();
            if let Some(mut next) = potential_next {
                let mut potential_other = next.borrow().next.clone();
                while let Some(another) = potential_other {
                    search = next;
                    next = another;
                    potential_other = next.borrow().next.clone();
                }
                let last = search;

                last.borrow_mut().next = None;

                self.last = Some(last);
            } else {

                self.first = None;
            }
        }

        Ok(Rc::try_unwrap(last).ok().expect(
        ).into_inner().item)
    }
}

fn cycle_dequeue(q: &mut Queue<usize>) {
    // Initially it is empty
    assert!(q.is_empty());
    assert_eq!(q.size(), 0);

    for _ in 1..10 {
        // Add elements to queue
        for i in 1..5 {
            q.enqueue(i);
            assert!(!q.is_empty());
            assert_eq!(q.size(), i);
        }
        // Take elements out
        for i in 1..5 {

            assert!(!q.is_empty());

            assert_eq!(q.dequeue(), Ok(i));
            assert_eq!(q.size(), 4-i);
        }
        assert!(q.is_empty());
    }
}

fn cycle_dequeue_back(q: &mut Queue<usize>) {
    assert!(q.is_empty());
    assert_eq!(q.size(), 0);

    for _ in 1..10 {
        // Add some elements
        for i in 1..5 {
            q.enqueue(i);
           // if this is not empty now..
            assert!(!q.is_empty());
            assert_eq!(q.size(), i);
        }
        for i in 1..5 {
            assert!(!q.is_empty());
            assert_eq!(q.dequeue_back(), Ok(5-i));

            assert_eq!(q.size(), 4-i);
        }
        // if poped the last element..
        assert!(q.is_empty());
    }
}

fn main() {
    // A new queue
    let mut q = Queue::new();

    cycle_dequeue(&mut q);
    cycle_dequeue_back(&mut q);

    // Pop some elements
    for _ in 1..17 {
        assert_eq!(q.dequeue_back(), Err("Queue is empty".to_owned()));
        assert_eq!(q.size(), 0);
        assert!(q.is_empty());
    }

    cycle_dequeue_back(&mut q);
    cycle_dequeue(&mut q)
}
