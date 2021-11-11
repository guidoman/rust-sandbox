use crossbeam_queue::ArrayQueue;
use crossbeam_utils::sync::ShardedLock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}
fn sharded_lock() {
    let lock = ShardedLock::new(5);
    // Any number of read locks can be held at once.
    {
        println!("Lock r1");
        let r1 = lock.read().unwrap();
        println!("Lock r2");
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // Read locks are dropped at this point.
      // However, only one write lock may be held.
    {
        println!("Lock w1");
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    } // Write lock is dropped here.
}

fn main() {
    let q = ArrayQueue::new(1);

    assert_eq!(q.push(10), Ok(()));
    assert_eq!(q.push(20), Err(20));
    println!("ok {}", q.len());
    
    let thread_handle = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let popped = q.pop();
            println!("wait {:?}", popped);
            
            if let Some(x) = popped {
                println!("some = {}", x);
            }
        }
    });

    thread_handle.join().expect("thread panicked");
    println!("done.");
}