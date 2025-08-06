fn main() {
    let mut head = initlist();

    insert(1, &mut head);
    insert(3, &mut head);
    insert(5, &mut head);

    display(&head);

    println!("Search for 1: {}", search(1, &head));
    println!("Search for 100: {}", search(100, &head));

    if let Some(delete_prev_node) = head.next.as_deref_mut() {
        delete(delete_prev_node);
    }

    display(&head);
}

struct List {
    value: i64,
    next: Option<Box<List>>,
}

fn initlist() -> List {
    let listhead = List {
        value: 0,
        next: None,
    };
    listhead
}

/// Time complexity: O(n)
/// Space complexity: O(1)
fn search(x: i64, listhead: &List) -> bool {
    let mut p = listhead.next.as_deref();
    
    while let Some(node) = p {
        if node.value == x {
            return true;
        }
        p = node.next.as_deref();
    }
    false
}

/// Time complexity: O(1)
/// Space complexity: O(1)
fn insert(x: i64, prev: &mut List) {
    let prev_next = prev.next.take();
    let p = Box::new(List {
        value: x,
        next: prev_next,
    });
    prev.next = Some(p);
}

/// Time complexity: O(1)
/// Space complexity: O(1)
fn delete(prev: &mut List) {
    if let Some(mut node) = prev.next.take() {
        prev.next = node.next.take();
    }
}

fn display(listhead: &List) {
    let mut p = listhead.next.as_deref();
    while let Some(node) = p{
        print!("{} ", node.value);
        p = node.next.as_deref();
    }
    println!();
}