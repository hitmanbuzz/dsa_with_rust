#[derive(Debug)]
struct Node<'a, T> {
    data: &'a T,
    next: Option<Box<Node<'a, T>>>,
}

struct SinglyLinkedList<'a, T> {
    head: Option<Box<Node<'a, T>>>
}

impl<'a, T: std::fmt::Debug + PartialEq> SinglyLinkedList<'a, T> {
    /// Create a new singly linked list
    /// 
    /// returns:
    ///     SinglyLinkedList<'a, T> - A new singly linked list
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    /// Push a new node to the front of the list
    /// 
    /// args:
    ///     data: &'a T - The data to be stored in the new node
    fn push_front(&mut self, data: &'a T) {
        let new_node = Node {
            data,
            next: self.head.take()
        };

        self.head = Some(Box::new(new_node));
    }

    /// Push a new node to the back of the list
    /// 
    /// args:
    ///     data: &'a T - The data to be stored in the new node
    fn push_back(&mut self, data: &'a T) {
        if self.head.is_none() {
            self.head = Some(Box::new(
                Node { 
                    data,
                    next: None 
                }
            ));
            return;
        }

        let mut curr = self.head.as_mut().unwrap();

        while let Some(ref mut value) = curr.next {
            curr = value;
        }

        curr.next = Some(Box::new(
            Node {
                data,
                next: None
            }
        ));
    }

    /// Remove the first node from the list
    /// 
    /// returns:
    ///     None - If the list is empty
    fn pop_front(&mut self) {
        if self.head.is_none() {
            println!("Linked List is empty");
            return;
        }

        self.head = self.head.take().unwrap().next;
    }

    /// Remove the last node from the list
    /// 
    /// returns:
    ///     Option<&'a T> - The data of the removed node
    fn pop_back(&mut self) -> Option<&'a T> {
        // If the list is empty, return None
        if self.head.is_none() {
            return None;
        }
        // If there's only one element
        if self.head.as_ref().unwrap().next.is_none() {
            // Take and return the data of the only node
            return self.head.take().map(|node| node.data);
        }

        // Traverse to find the second to last node
        let mut current = self.head.as_mut().unwrap();
        while current.next.as_ref().unwrap().next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        // Remove the last node and return its data
        current.next.take().map(|node| node.data)
    }

    /// Insert a new node at a specific index
    /// 
    ///     args:
    /// 
    ///     index: u32 - The index to insert the new node at
    ///     data: &'a T - The data to be stored in the new node
    fn insert_at_index(&mut self, index: u32, data: &'a T) {
        if index > self.get_length() {
            println!("Index out of range");
            return;
        }

        if index == 0 {
            self.push_front(data);
            return;
        }

        if index == self.get_length() {
            self.push_back(data);
            return;
        }

        let mut index_counter = 0;
        let mut curr = self.head.as_mut().unwrap();

        while index_counter < index - 1 {
            curr = curr.next.as_mut().unwrap();
            index_counter += 1;
        }

        let new_node = Node {
            data,
            next: curr.next.take()
        };

        curr.next = Some(Box::new(new_node));
    }

    /// Get the length of the list
    /// 
    /// returns:
    ///     u32 - The length of the list
    fn get_length(&self) -> u32 {
        let mut counter: u32 = 0;
        let mut curr = self.head.as_ref();
        while let Some(value) = curr {
            curr = value.next.as_ref();
            counter += 1;
        }

        return counter;
    }

    /// Find a node with a specific data
    /// 
    /// args:
    ///     data: &'a T - The data to be searched for
    /// 
    /// returns:
    ///     (bool, u32) - A tuple containing a boolean and the index of the node
    fn find(&self, data: &'a T) -> (bool, u32)
    where
        T: PartialEq
    {
        let mut curr = self.head.as_ref();
        let mut index_counter: u32 = 0;
        while let Some(value) = curr {
            if value.data == data {
                return (true, index_counter);
            }
            index_counter += 1;
            curr = value.next.as_ref();
        }

        return (false, 0);
    }

    /// Insert a new node after a specific node
    /// 
    /// args:
    ///     node: &'a T - The node to insert the new node after
    /// 
    ///    data: &'a T - The data to be stored in the new node
    fn insert_after(&mut self, node: &'a T, data: &'a T) {
        if self.head.is_none() {
            println!("Linked List is empty");
            return;
        }

        let mut curr = self.head.as_mut().unwrap();
        while curr.data != node {
            curr = curr.next.as_mut().unwrap();
        }
        
        let new_node = Node {
            data,
            next: curr.next.take()
        };

        curr.next = Some(Box::new(new_node));
    }

    /// Check if the list is empty
    /// 
    /// returns:
    ///     bool - True if the list is empty, false otherwise
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Display the list
    /// 
    /// where:
    ///     T: std::fmt::Display - The type of the data in the list
    fn display(&self)
    where 
        T: std::fmt::Display
    {
        let mut curr = self.head.as_ref();
        while let Some(value) = curr {
            print!("{} -> ", value.data);
            curr = value.next.as_ref();
        }
        println!("None");
    }

    /// Reverse the list
    fn reverse(&mut self) {
        let mut prev: Option<_> = None;
        let mut curr = self.head.take();

        while let Some(mut boxed_node) = curr {
            let next = boxed_node.next;
            boxed_node.next = prev;
            prev = Some(boxed_node);
            curr = next;
        }

        self.head = prev;
    }
}

/// Run the program
pub fn run() {
    let mut singly_ll = SinglyLinkedList::new();
    singly_ll.push_front(&10);
    singly_ll.push_front(&20);
    singly_ll.push_front(&30);

    singly_ll.display();

    singly_ll.push_back(&50);
    singly_ll.push_back(&80);

    singly_ll.display(); 

    singly_ll.pop_front();
    singly_ll.display();

    if let Some(last_value) = singly_ll.pop_back() {
        println!("Popped Value: {}", last_value);
    }
    singly_ll.display();

    singly_ll.insert_at_index(2, &100);
    singly_ll.display();

    singly_ll.insert_after(&20, &100);
    singly_ll.display();

    singly_ll.insert_after(&20, &500);
    singly_ll.display();

    let (found, index) = singly_ll.find(&50);
    if found {
        println!("Found at index: {}", index);
    } else {
        println!("Not found");
    }
    
    if singly_ll.is_empty() {
        println!("Linked List is empty");
    } else {
        println!("Linked List is not empty");
    }

    singly_ll.reverse();
    singly_ll.display();
}