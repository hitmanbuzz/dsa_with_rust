use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Node<'a, T> {
    data: &'a T,
    next: Option<Rc<RefCell<Node<'a, T>>>>,
    prev: Option<Rc<RefCell<Node<'a, T>>>>,
}

struct DoublyLinkedList<'a, T> {
    head: Option<Rc<RefCell<Node<'a, T>>>>,
    tail: Option<Rc<RefCell<Node<'a, T>>>>,
}

impl<'a, T: std::fmt::Debug + PartialEq + Copy + Clone> DoublyLinkedList<'a, T> {
    /// Create a new doubly linked list
    /// 
    /// Will create a new doubly linked list with no nodes
    fn new() -> Self {
        DoublyLinkedList { head: None, tail: None }
    }

    /// Push a new node to the back of the list
    /// 
    /// Will create a new node with the given data and add it to the end of the list
    /// args:
    /// * `data`: The data to be added to the list
    fn push_front(&mut self, data: &'a T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None
        }));
    
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                let new_head = Rc::new(RefCell::new(Node {
                    data,
                    next: Some(old_head),
                    prev: None
                }));
                self.head = Some(new_head);
            }
            None => {
                // List is empty
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    fn display(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{:?} -> ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!();
    }
}


/// Run the program
pub fn run() {
    let mut doubly_ll = DoublyLinkedList::new();

    doubly_ll.display();
    doubly_ll.push_front(&10);
    doubly_ll.push_front(&20);
    doubly_ll.push_front(&30);
    doubly_ll.push_front(&40);

    doubly_ll.display();
}