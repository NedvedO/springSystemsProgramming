use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter wrapped in Arc<Mutex<>>
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _i in 1..=5 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                // MutexGuard drops here, releasing the lock each iteration
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
    // Expected: 50 (5 threads × 10 increments)
}