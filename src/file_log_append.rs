use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
fn main() {
    {
        let mut file = OpenOptions::new()
            // .create_new(true)
            .append(true)
            .write(true)
            .create(true)
            .open("data.txt")
            .expect("cannot open file");
        file.write_all("Hello World".as_bytes())
            .expect("write failed");
        file.write_all("\nTutorialsPoint".as_bytes())
            .expect("write failed");
        println!("file append success");
    }

    {
        let metadata = fs::metadata("data.txt").expect("cannot get metadata");
        println!("file len = {:?}", metadata.len());
    }
    let offset: u64 = 5;
    {
        let mut file = File::open("data.txt").unwrap();
        file.seek(SeekFrom::Start(offset))
            .expect("cannot seek file");
    }
    {
        let mut file = File::open("data.txt").unwrap();
        file.seek(SeekFrom::Start(offset))
            .expect("cannot seek file");
        let mut res = String::new();
        let length: u64 = 3;
        let mut handle = file.take(length).read_to_string(&mut res);
        println!("Read data: [{:?}]", res);
    }
    {
        // let mut file = File::create("data.bin").expect("cannot create out file");
        let mut file = OpenOptions::new()
            .append(true)
            .open("data.bin")
            .expect("cannot open file");
        // Write a slice of bytes to the file
        let v: u8 = 3;

        println!("TODO write binary data");
        // file.write_all(&[v]).expect("cannot write file");
    }
    {
        let mut file = File::open("data.bin").expect("cannot open file");
        // read the same file back into a Vec of bytes
        let mut buffer = Vec::<u8>::new();
        // file.read_to_end(&mut buffer)
        //     .expect("cannot read from binary file");

        let offset: u64 = 1;
        file.seek(SeekFrom::Start(offset))
            .expect("cannot seek file");
        // let mut buffer = [0; 10];
        // file.read_exact(&buffer);
        file.take(1).read_to_end(&mut buffer).expect("cannot read data");

        println!("read data = {:?}, size = {:?}", buffer, buffer.len());
    }
}
