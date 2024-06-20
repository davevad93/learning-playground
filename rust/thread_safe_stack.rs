use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

// A thread-safe stack implementation
struct Stack<T> {
    data: Mutex<Vec<T>>,
    size: AtomicUsize,
}

impl<T> Stack<T> {
    // Create a new, empty stack
    fn new() -> Self {
        Stack {
            data: Mutex::new(Vec::new()),
            size: AtomicUsize::new(0),
        }
    }

    // Push an item onto the stack
    fn push(&self, item: T) {
        let mut data = self.data.lock().unwrap();
        data.push(item);
        self.size.fetch_add(1, Ordering::SeqCst);
    }

    // Pop an item off the stack
    fn pop(&self) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        if let Some(item) = data.pop() {
            self.size.fetch_sub(1, Ordering::SeqCst);
            Some(item)
        } else {
            None
        }
    }

    // Get the current size of the stack
    fn size(&self) -> usize {
        self.size.load(Ordering::SeqCst)
    }
}

fn main() {
    // Create a new stack
    let stack = Arc::new(Stack::new());

    // Spawn a few threads to work with the stack
    let mut handles = vec![];
    for i in 0..5 {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                stack.push(i * 10 + j);
                println!("Thread {} pushed {}", i, i * 10 + j);
            }
        });
        handles.push(handle);
    }

    // Join all the threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Pop all items from the stack
    while let Some(item) = stack.pop() {
        println!("Popped {}", item);
    }

    // Print the final size of the stack
    println!("Final size of the stack: {}", stack.size());
}
