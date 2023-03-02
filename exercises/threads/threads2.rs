// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

// 1. Imported Mutex
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 2. Wrapped JobStatus inside of Mutex
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 3. Took a lock before updating the shared value
            let mut status_shared = status_shared
                .lock()
                .expect("Failed to acquire a lock on the Mutex from the Arc.");
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // 4. Print the value of the JobStatus.jobs_completed
        let status = status
            .lock()
            .expect("Failed to acquire a lock on the Mutex");
        println!("jobs completed {}", status.jobs_completed);
    }
}
