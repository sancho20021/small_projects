use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    const MAX_THREADS: usize = 80;
    let counter = Arc::new(Mutex::new(1));
    let max = Arc::new(Mutex::new(1));

    let mut handles = Vec::with_capacity(MAX_THREADS);

    for i in 0..MAX_THREADS {
        let counter_clone = Arc::clone(&counter);
        let max_clone = Arc::clone(&max);
        let handle = thread::spawn(move || {
            {
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
                let mut max = max_clone.lock().unwrap();
                *max = std::cmp::max(*max, *count);
            }

            // Simulate work by sleeping
            thread::sleep(Duration::from_secs(3));

            {
                let mut count = counter_clone.lock().unwrap();
                *count -= 1;
            }
        });
        handles.push(handle);
    }

    // Join all threads before exiting
    for handle in handles {
        handle.join().unwrap();
    }
    let max = *max.lock().unwrap();

    println!("All threads finished, max simultaneous threads = {max}");
}
