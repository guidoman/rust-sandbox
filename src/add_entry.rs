use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Write};
use std::collections::HashMap;
use byteorder::{ByteOrder, LittleEndian};

fn add_entry(key: u16, json_value: &str, mem_table: &mut HashMap<u16, u64>, file_path: &str) {
    println!("Adding entry with key [{}] to file [{}]", key, file_path);
    let content_num_bytes= String::from(json_value).len() as u64;
    println!(
        "JSON value n. bytes: [{}]. Content: [{}]",
        content_num_bytes, json_value
    );
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("cannot open file");

    let metadata = fs::metadata(file_path).expect("cannot get metadata");
    let orig_file_size = metadata.len();
    println!("file len = {:?}", metadata.len());
    
    // key
    let mut buf = [0; 2];
    LittleEndian::write_u16(&mut buf, key);
    file.write_all(&buf).expect("cannot write to file");

    // content size
    let mut buf = [0; 8];
    LittleEndian::write_u64(&mut buf, content_num_bytes);
    file.write_all(&buf).expect("cannot write to file");

    // content
    file.write_all(json_value.as_bytes()).expect("cannot write to file");

    mem_table.insert(key, orig_file_size);
}

fn main() {
    // let data = r#"{
    //     "name": "John Doe",
    //     "age": 43,
    //     "phones": [
    //       "+44 1234567",
    //       "+44 2345678"
    //     ]
    //   }"#;
    // let data = "ラウトは難しいです！";
    let mut mem_table : HashMap<u16, u64> = HashMap::new();

    let data = r#"{"name":"Guido"}"#;
    println!("Data n. chars: {}", data.chars().count());
    add_entry(123, data, &mut mem_table, "1.guidcask");
    
    
    println!("{:?}", mem_table);
}
