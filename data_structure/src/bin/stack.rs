fn main() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    stack.display();
    println!("pop -> {:?}", stack.pop());
    stack.display();
}

struct Node {
    value: i64,
    next: Option<Box<Node>>,
}

struct Stack {
    top: Option<Box<Node>>,
}

impl Stack {
    fn new() -> Self { Stack { top: None }}

    /// Time complexity: O(1)
    /// Space complexity: O(1)
    fn push(&mut self, x: i64) {
        let node = Node { value: x, next: self.top.take() };
        self.top = Some(Box::new(node));
    }

    /// Time complexity: O(1)
    /// Space complexity: O(1)
    fn pop(&mut self) -> Option<i64> {
        self.top.take().map(|top| {
            let Node { value, next} = *top;
            self.top = next;
            value
        })
    }

    fn display(&self) {
        let mut p = self.top.as_deref();
        println!("top -> bottom");
        while let Some(node) = p {
            print!("{} ", node.value);
            p = node.next.as_deref();
        }
        println!();
    }
}