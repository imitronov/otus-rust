use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: u32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: u32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Debug)]
struct List {
    head: Option<Rc<RefCell<Node>>>,
}

impl List {
    fn new(value: u32) -> Self {
        List {
            head: Some(Node::new(value)),
        }
    }

    fn append_after(&mut self, value: u32, index: isize) {
        let mut current_index = 0;
        let mut current = self.head.clone().expect("List is empty");

        while current_index < index {
            let next = current.borrow().next.clone().expect("Index out of range");
            current = next;
            current_index += 1;
        }

        let new_node = Node::new(value);
        new_node.borrow_mut().next = current.borrow_mut().next.take();
        current.borrow_mut().next = Some(new_node);
    }

    fn iter(&mut self) -> ListIterator {
        ListIterator {
            current: self.head.clone(),
        }
    }
}

struct ListIterator {
    current: Option<Rc<RefCell<Node>>>,
}

impl Iterator for ListIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let node_rc = self.current.as_ref()?.clone();

        let value = node_rc.borrow().value;
        self.current.clone_from(&node_rc.borrow().next.clone());

        Some(value)
    }
}

fn main() {
    let mut list = List::new(20);
    list.append_after(22, 0);
    list.append_after(23, 1);
    list.append_after(21, 0);

    for node in list.iter() {
        println!("{:?}", node);
    }

    list.append_after(21, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_iter() {
        let mut list = List::new(1);

        list.append_after(2, 0);
        list.append_after(3, 1);

        let values: Vec<_> = list.iter().collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_insert_after() {
        let mut list = List::new(10);

        list.append_after(20, 0);
        list.append_after(30, 1);
        list.append_after(40, 2);

        list.append_after(15, 0);

        let values: Vec<_> = list.iter().collect();
        assert_eq!(values, vec![10, 15, 20, 30, 40]);
    }

    #[test]
    #[should_panic(expected = "Index out of range")]
    fn test_insert_out_of_bounds_panics() {
        let mut list = List::new(1);
        list.append_after(2, 5);
    }
}
