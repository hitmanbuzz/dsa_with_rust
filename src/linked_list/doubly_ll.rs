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

#[allow(dead_code)]
impl<'a, T: std::fmt::Debug + PartialEq + Copy + Clone + std::fmt::Display> DoublyLinkedList<'a, T> {
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

    /// Push a new node to the back of the list
    /// 
    /// Will create a new node with the given data and add it to the end of the list
    /// 
    /// args:
    /// * `data`: The data to be added to the list
    fn push_back(&mut self, data: &'a T) {
       if self.head.is_none() {
            println!("List is empty");
            return;
        }
        if let Some(tail) = &self.tail {
            let new_node = Rc::new(RefCell::new(Node {
                data, // Placeholder data
                next: None,
                prev: Some(tail.clone())
            }));
            
            tail.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }
    
    /// Insert a node/data after a certain node/data.
    /// 
    /// args:
    /// * `_node`: The node required to push for `data`
    ///
    /// * `data`: The node/data to be inserted
    fn push_after_node(&mut self, _node: &'a T, data: &'a T) {
        if self.head.is_none() {
            println!("List is empty");
            return;
        }

        let mut current = self.head.clone();

        while let Some(ref node) = current.clone() {
            // println!("Node: {}", &node.borrow_mut().data);
            if node.borrow_mut().data == _node {
                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    next: node.borrow().next.clone(),
                    prev: node.borrow().prev.clone(), 
                }));
                
                node.borrow_mut().next = Some(new_node);
            }            
            
            current = node.borrow_mut().next.clone();
        } 
    }

    /// Finding a node/data in a list and return the index of the node from the list
    /// 
    /// return: 
    /// * `(bool, u32)` -> `(true, index)` | `(false, 0)`
    fn find(&self, data: &'a T) -> (bool, u32) {
        let mut current = self.head.clone();
        let mut index_counter: u32 = 0;

        while let Some(node) = current {
            if node.borrow().data == data {
                return (true, index_counter);
            }
            index_counter += 1;
            current = node.borrow().next.clone();
        } 
        return (false, 0);
    }
    
    /// Get the length (total no. of items in the list)
    /// 
    /// return: 
    /// * `u32` -> The total number of items in the list
    fn get_length(&self) -> u32 {
        let mut current = self.head.clone();
        let mut counter: u32 = 0;

        while let Some(node) = current {
            counter += 1;
            current = node.borrow().next.clone();
        }
        
        return counter;
    }  

    /// Display the list
    fn display(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{:?} -> ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!();
    }
    
    /// Get the value of the tail node
    fn get_tail_value(&self) {
        if let Some(tail) = &self.tail {
            println!("Tail value: {:?}", tail.borrow().data);
        } else {
            println!("List is empty");
        }
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
    
    doubly_ll.push_back(&50);
    doubly_ll.push_back(&60);
    doubly_ll.display();
    doubly_ll.get_tail_value();
    
    doubly_ll.push_back(&70);
    doubly_ll.push_back(&80);
    doubly_ll.display();
    doubly_ll.get_tail_value();
    
    doubly_ll.push_after_node(&50, &969);
    doubly_ll.display();
    

    doubly_ll.push_after_node(&969, &1000);
    doubly_ll.display();
    

    println!("List Length: {}", doubly_ll.get_length());
}