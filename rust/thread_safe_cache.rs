use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// Define a thread-safe cache
struct Cache<K, V> {
    data: RwLock<HashMap<K, V>>,
}

impl<K, V> Cache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,  // Key must implement Eq, Hash, and Clone
    V: Clone,                                   // Value must implement Clone
{
    // Create a new, empty cache
    fn new() -> Self {
        Cache {
            data: RwLock::new(HashMap::new()),
        }
    }

    // Insert a key-value pair into the cache
    fn insert(&self, key: K, value: V) {
        let mut write_lock = self.data.write().unwrap();
        write_lock.insert(key, value);
    }

    // Retrieve a value from the cache by key
    fn get(&self, key: &K) -> Option<V> {
        let read_lock = self.data.read().unwrap();
        read_lock.get(key).cloned()
    }
}

fn main() {
    // Create a new cache
    let cache = Arc::new(Cache::new());

    // Spawn a thread to insert values into the cache
    let cache_producer = Arc::clone(&cache);
    let producer_handle = thread::spawn(move || {
        for i in 0..10 {
            println!("Producer: Inserting key {} with value {}", i, i * 2);
            cache_producer.insert(i, i * 2);
            thread::sleep(Duration::from_millis(100)); // Simulate work
        }
    });

    // Spawn a thread to retrieve values from the cache
    let cache_consumer = Arc::clone(&cache);
    let consumer_handle = thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(150)); // Simulate delay
            if let Some(value) = cache_consumer.get(&i) {
                println!("Consumer: Retrieved value {} for key {}", value, i);
            } else {
                println!("Consumer: No value found for key {}", i);
            }
        }
    });

    // Wait for both threads to finish
    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();

    println!("Cache operations completed.");
}
