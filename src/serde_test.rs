use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Serialize, Deserialize, Debug)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    // moved_points: Vec<(u32, u32)>,
    moved_points: Vec<Point>,
    dir: MoveDirection,
}

fn main() -> Result<()> {
    let a = Move {
        moved_points: vec![Point { x: 1, y: 1 }, Point { x: 2, y: 2 }],
        dir: MoveDirection::Up,
    };
    let j = serde_json::to_string(&a)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", &j);

    let write_file = File::create("serde.json").expect("cannot create output file");
    let mut writer = BufWriter::new(&write_file);
    write!(&mut writer, "{}", j).expect("cannot write to file");

    let a: Move = serde_json::from_str(&*j)?;
    println!("{:?}", a);
    // let x = (1, 1);
    // let v: Vec<(i32, i32)> = vec![(1, 1), (2, 2)];

    Ok(())
}
