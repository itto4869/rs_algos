use std::ptr::NonNull;

fn main() {
    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(3);
    queue.enqueue(5);

    let mut p = &queue.head;
    println!("head -> tail");
    while let Some(node) = p {
        print!("{} ", node.value);
        p = &node.next;
    }
    println!();

    queue.dequeue();
    queue.dequeue();

    let mut p = &queue.head;
    println!("head -> tail");
    while let Some(node) = p {
        print!("{} ", node.value);
        p = &node.next;
    }
    println!();
}

struct Node {
    value: i64,
    next: Option<Box<Node>>,
}

struct Queue {
    head: Option<Box<Node>>,
    tail: Option<NonNull<Node>>,
}

impl Queue {
    fn new() -> Self { Queue {
        head: None,
        tail: None,
    }
    }

    /// Time complexity: O(1)
    /// Space complexity: O(1)
    fn enqueue(&mut self, x: i64) {
        let mut node = Box::new(Node {
            value: x,
            next: None,
        });

        let new_tail = unsafe { NonNull::new_unchecked(node.as_mut()) };

        if let Some(mut tail) = self.tail {
            unsafe { (tail.as_mut()).next = Some(node); }
        } else {
            self.head = Some(node);
        }

        self.tail = Some(new_tail);
    }

    /// Time complexity: O(1)
    /// Space complexity: O(1)
    fn dequeue(&mut self) -> Option<i64> {
        self.head.take().map(|head| {
            let Node { value, next} = *head;
            self.head = next;
            if self.head.is_none() {
                self.tail = None;
            }
            value
        })
    }
}