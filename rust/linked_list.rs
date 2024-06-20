use std::ptr;

// Define a simple Node struct for a linked list
struct Node {
    data: i32,
    next: *mut Node,
}

impl Node {
    // Constructor function for Node
    fn new(data: i32) -> Self {
        Node { data, next: ptr::null_mut() }
    }
}

// Define a LinkedList struct to manage the list
struct LinkedList {
    head: *mut Node,
}

impl LinkedList {
    // Create a new, empty LinkedList
    fn new() -> Self {
        LinkedList { head: ptr::null_mut() }
    }

    // Insert a new node at the beginning of the list
    fn insert(&mut self, data: i32) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head;
        self.head = Box::into_raw(new_node);
    }

    // Print the contents of the list
    fn print_list(&self) {
        let mut current_node = self.head;
        while !current_node.is_null() {
            unsafe {
                println!("Node data: {}", (*current_node).data);
                current_node = (*current_node).next;
            }
        }
    }
}

fn main() {
    // Create a new linked list
    let mut list = LinkedList::new();

    // Insert nodes into the list
    list.insert(1);
    list.insert(2);
    list.insert(3);

    // Print the linked list
    println!("Initial linked list:");
    list.print_list();

    // Insert a new node at the beginning of the list
    list.insert(0);

    // Print the modified linked list
    println!("Modified linked list:");
    list.print_list();
}
