// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed


use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

#[derive(Debug)]
struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        // create clone of status
        let status_shared = status.clone();
        // create thread to run unwrap.completed += 1
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // increment more 1 in job_completed
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        // add process to status arc.
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        // use the .lock to blocking the code in this point and take the owner to print.
        println!("jobs completed {:?}", *status.lock().unwrap());
    }
}
