use crossbeam_channel::bounded;
use std::thread;

fn main() {
    let pool_size = 4;
    let (s, r) = bounded(pool_size);

    for i in 0..pool_size {
        let r = r.clone();
        thread::spawn(move || loop {
            // if let Ok(received) = r.recv() {
            //     println!("Thread {} - Received {:?}", i, received());
            // }
            match r.recv() {
                Ok(received) => {
                    // println!("Thread {} - Received {:?}", i, "todo");
                    // println!("Thread {} - Received {:?}", i, received());
                }
                
                ,
                Err(err) => {
                    println!("Thread {} - error: {:?}", i, err);
                }
            }
        });
    }

    for i in 0..100 {
        s.send( move || {i}).unwrap();
    }
}
