#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::DirEntry, io, path::Path};
use std::time::Duration;
use std::thread::sleep;

fn path_test() -> io::Result<()> {
    let path = Path::new("/Users/guido/Downloads/guidcask_data_test");
    let dir = path.read_dir()?;
    let dir_entries: Vec<DirEntry> = dir.into_iter().filter_map(|x| x.ok()).collect();
    
    for entry in dir_entries {
        // let fname = entry.file_name().;

    }

    return Ok(());
}

fn main() {
    println!("something ...");
    // path_test().expect("path_test failed");

    let handle_1 = std::thread::spawn(|| {
        loop {
            println!("Thread 1");
            sleep(Duration::from_millis(1000));
        }
    });
    let handle_2 = std::thread::spawn(|| {
        loop {
            println!("Thread 2");
            sleep(Duration::from_millis(1001));
        }
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    println!("Will never get here");
}
