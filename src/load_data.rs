use std::fs;
use std::str;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use byteorder::{ByteOrder, LittleEndian};

fn read_entry(file: &mut std::fs::File, offset: u64) -> u64 {
    let mut total_read_size = 0;
    file.seek(SeekFrom::Start(offset)).expect("cannot seek file");

    let mut buf = [0; 2];
    let read_size = file.read(&mut buf).unwrap();
    println!("Read size for key = {}", read_size);
    let key = LittleEndian::read_u16(&buf);
    println!("key = {}", key);
    total_read_size += read_size as u64;

    // content size
    // offset += 2;
    file.seek(SeekFrom::Start(offset + total_read_size)).expect("cannot seek file");
    let mut buf = [0; 8];
    let read_size = file.read(&mut buf).unwrap();
    println!("Read size = {}", read_size);
    let content_size = LittleEndian::read_u64(&buf);
    println!("content size = {}", content_size);
    total_read_size += read_size as u64;

    // content
    // Read "content size" bytes, decode to string
    let mut buf = vec![0u8; content_size as usize];
    file.read_exact(&mut buf).unwrap();
    let content = str::from_utf8(&buf).unwrap();
    println!("{:?}", content);
    total_read_size += content_size;

    return total_read_size;
}


fn main() {
    println!("load_data");
    let mut file = File::open("1.guidcask").unwrap();

    // key
    let mut offset : u64 = 0;
    

    let read_size = read_entry(&mut file, offset);
    println!("First entry read size = {}", read_size);

    offset += read_size;

    let read_size = read_entry(&mut file, offset);
    println!("Second entry read size = {}", read_size);

}
