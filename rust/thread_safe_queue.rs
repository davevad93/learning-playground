use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

// Define a thread-safe generic queue
struct Queue<T> {
    data: Mutex<Vec<T>>,
    condvar: Condvar,
}

impl<T> Queue<T> {
    // Create a new, empty queue
    fn new() -> Self {
        Queue {
            data: Mutex::new(Vec::new()),
            condvar: Condvar::new(),
        }
    }

    // Enqueue an item to the queue
    fn enqueue(&self, item: T) {
        let mut data = self.data.lock().unwrap();
        data.push(item);
        self.condvar.notify_one();
    }

    // Dequeue an item from the queue
    fn dequeue(&self) -> T {
        let mut data = self.data.lock().unwrap();
        loop {
            if let Some(item) = data.pop() {
                return item;
            } else {
                data = self.condvar.wait(data).unwrap();
            }
        }
    }
}

fn main() {
    // Create a new queue
    let queue = Arc::new(Queue::new());

    // Spawn a producer thread
    let queue_producer = Arc::clone(&queue);
    let producer_handle = thread::spawn(move || {
        for i in 0..10 {
            println!("Producer: Enqueueing {}", i);
            queue_producer.enqueue(i);
            thread::sleep(Duration::from_millis(100)); // Simulate work
        }
    });

    // Spawn a consumer thread
    let queue_consumer = Arc::clone(&queue);
    let consumer_handle = thread::spawn(move || {
        for _ in 0..10 {
            let item = queue_consumer.dequeue();
            println!("Consumer: Dequeued {}", item);
        }
    });

    // Wait for both threads to finish
    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();

    println!("All items have been processed.");
}
