use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node<'a, T> {
    data: &'a T,
    next: Option<Rc<RefCell<Node<'a, T>>>>,
}

/// Code structure for the Circular Linked List
struct CircularLinkedList<'a, T> {
    head: Option<Rc<RefCell<Node<'a, T>>>>,
}

#[allow(dead_code)]
impl<'a, T> CircularLinkedList<'a, T> {
    /// Create a new empty circular linked list
    fn new() -> Self {
        Self { head: None }
    }

    /// Insert a element at the front/start of the list
    /// 
    /// args:
    /// * `data`: the data that will be inserted `(&'a T)`
    fn push_front(&mut self, data: &'a T)
    {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: None
        }));

        match &self.head {
            Some(head) => {
                new_node.borrow_mut().next = Some(head.clone());

                let mut curr = head.clone();
                while let Some(value) = curr.clone().borrow().next.clone() {
                    if Rc::ptr_eq(&value, head) {
                        break;
                    }

                    curr = value;
                }
                

                curr.borrow_mut().next = Some(new_node.clone());
            }
            None => {
                new_node.borrow_mut().next = Some(new_node.clone())
            }
        }

        self.head = Some(new_node);
    }


    /// Print all the items/data of the list
    fn display(&self)
    where T: std::fmt::Display
    {
        if let Some(head) = &self.head {
            let mut current = head.clone();
            loop {
                print!("{} -> ", current.borrow().data);
                let next = current.borrow().next.clone();
                
                if let Some(next_node) = next {
                    if Rc::ptr_eq(&next_node, head) {
                        break;
                    }
                    current = next_node;
                }
            }
            println!("(back to start)");
        } else {
            println!("Empty list");
        }
    }

    /// Check if the list is empty or not
    /// 
    /// It doesn't return any type because it is use for instant check
    fn is_empty(&self) {
        if self.head.is_none() {
            println!("Linked List is empty");
            return;
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let mut circular_ll = CircularLinkedList::new();

    circular_ll.push_front(&50);
    circular_ll.push_front(&60);

    circular_ll.display();
}