// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // NOTE: Added the Arc and Mutex types to the JobStatus struct, and to the status variable
    // below to make it thread-safe and shareable between threads.
    // Arc is a thread-safe reference-counting pointer
    // Mutex is a mutual exclusion primitive useful for protecting shared data
    // https://doc.rust-lang.org/std/sync/struct.Arc.html
    // https://doc.rust-lang.org/std/sync/struct.Mutex.html
    // https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    let start_time = std::time::Instant::now();
    for _ in 0..10 {
        let status_shared = status.clone(); // NOTE: same as, Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            {
                status_shared.lock().unwrap().jobs_completed += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        // NOTE: The join() method will block the current thread until the thread represented by the handle terminates.
        // https://doc.rust-lang.org/std/thread/struct.JoinHandle.html
        if status.lock().unwrap().jobs_completed == 10 {
            break;
        } else {
            handle.join();
        }
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }

    let elapsed_time = start_time.elapsed();
    println!(
        "Elapsed time: {:?}, jobs_completed: {}",
        elapsed_time,
        status.lock().unwrap().jobs_completed
    );
}
