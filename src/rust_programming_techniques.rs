#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{Arg, App, SubCommand};

// ----- If you have Some thing, do Some thing with it, else return None

fn add_four(x: i32) -> i32 {
    x + 4
}

fn maybe_add_four_worst(y: Option<i32>) -> Option<i32> {
    match y {
        Some(yy) => Some(add_four(yy)),
        None => None
    }
}

fn maybe_add_four_better(y: Option<i32>) -> Option<i32> {
    y.map(add_four)
}

fn maybe_add_four_best(y: Option<i32>) -> Option<i32> {
    y.map(|x| x+4)
}

// ------
// and, and_then, or, or_else 
// ------
// ok_or, iter

use std::{error::Error, fmt};

#[derive(Debug)]
struct ErrNegative;

impl Error for ErrNegative {}

impl fmt::Display for ErrNegative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I'm sorry, it's negative")
    }
}

fn foo_bad(input: Option<i32>) -> Option<i32> {
    if input.is_none() {
        return None;
    }
    let input = input.unwrap();
    if input < 0 {
        return None;
    }
    Some(input)
}

fn bar_bad(input: Option<i32>)-> Result<i32, ErrNegative> {
    match foo_bad(input) {
        Some(n) => Ok(n),
        None => Err(ErrNegative)
    }
}

fn foo_good(input: Option<i32>) -> Option<i32> {
    // Rust nightly
    // input.filter(|x| x >= 0)
    input.and_then(|x| {
        if x < 0 {
            None
        } else {
            Some(x)
        }
    })
}

fn bar_good(input: Option<i32>) -> Result<i32, ErrNegative> {
    foo_good(input).ok_or(ErrNegative)
}

fn fun_with_iterators() {
    let vec = vec![0, 1, 2, 3, 4, 5];

    for v in &vec {
        println!("V1: {}", v);
    }
    vec.iter().for_each(|v| println!("V2: {}", v));
    (&vec).into_iter().for_each(|v| println!("V3: {}", v));
    
    let mut iter = (&vec).into_iter();
    while let Some(v) = iter.next() {
        println!("V4: {}", v);
    }
    
    let mut iter = (&vec).into_iter();
    loop {
        let v = match iter.next() {
            Some(v) => v,
            None => break
        };
        println!("V5: {}", v)
    }
}

fn main() {
    let y = 42;
    maybe_add_four_worst(Some(y));
    maybe_add_four_better(Some(y));
    maybe_add_four_best(Some(y));

    fun_with_iterators();
}
