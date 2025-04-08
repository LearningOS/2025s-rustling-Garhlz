use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 Arc 和 Mutex 包裹 JobStatus
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 锁定 Mutex，更新共享值
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        handle.join().unwrap();
        println!("thread {} completed, total jobs: {}", i, status.lock().unwrap().jobs_completed);
    }
}