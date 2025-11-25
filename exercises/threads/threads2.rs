// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


// Arc can determine the security of shared data between threaeds.
// However, if I want to update this data, I have to be sure that it will not be modified by multiple threads in the same time.
// I have to use Mutex to handle it.
// Yeah, std::sync::Mutex;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // TODO：修改共享数据前，必须先获取 Mutex 的锁
            let mut status_data = status_shared.lock().expect("Error: Can't get the data, dead lock may occurred!");
            status_data.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        // let all the handle completed, and then let's print what we want.
        // Yeah, do you like what you see?

    }

    let final_status_data = status.lock().unwrap();
    println!("jobs completed: {}", final_status_data.jobs_completed);
}
