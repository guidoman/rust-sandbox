#![allow(dead_code)]
#![allow(unused_variables)]

// use std::marker::Copy;
use std::{fs::DirEntry, io, path::Path};
use std::time::Duration;
use std::thread::sleep;


#[derive(Copy, Clone, Debug)]
struct MyStruct {
    my_int: i32
}



fn my_fn(s: MyStruct) -> i32 {
    s.my_int * 2
}


fn take_closure(closure: &dyn Fn()) {
    println!("Invoking closure...");
    closure();
}

fn main() {
    let s = MyStruct { my_int: 35};

    println!("before {:?}", s);
    let res1 = my_fn(s);
    
    let clos = move || {println!("inside closure = {:?}", s)};
    // let clos = move || {println!("closure = {:?}", 123)};
    
    take_closure(&clos);

    println!("at end {:?}", s);

}
