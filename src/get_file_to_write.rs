use std::fs;
use std::path::Path;

use tempfile::tempdir;
use std::fs::File;
use std::io::{self, Write};

static DATA_FILE_EXT: &str = "guidcask";
static MAX_FILE_SIZE : usize = 1024 * 1024 * 1024;

fn get_file_to_write(data_dir: &Path, new_data_size: usize) -> Result<String, &str> {
    if let Ok(dir) = data_dir.read_dir() {
        let mut max_file_id: u16 = 0;
        for entry in dir {
            if let Ok(entry) = entry {
                let pb = entry.path();
                let entry_path = pb.as_path();
                if entry_path.extension().unwrap() == DATA_FILE_EXT {
                    let progr_id = entry_path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .parse::<u16>()
                        .unwrap();
                    if progr_id > max_file_id {
                        max_file_id = progr_id;
                    }
                    println!(
                        "file stem = {:?}, extension = {:?}",
                        entry_path.file_stem(),
                        entry_path.extension()
                    );
                }
            }
        }
        if max_file_id == u16::MAX {
            return Err("Number of data files reached maximum allowed");
        }
        if max_file_id == 0 {
            // No file found, start from 1
            let new_file_path = format!("1.{ext}", ext = DATA_FILE_EXT);
            return Ok(new_file_path);
        }
        let curr_file = format!(
            "{dir}/{file_num}.{ext}",
            dir = data_dir.to_str().unwrap(),
            file_num = max_file_id,
            ext = DATA_FILE_EXT
        );
        let mut file_id_to_return = max_file_id;
        let metadata = fs::metadata(curr_file).unwrap();
        if metadata.len() as usize + new_data_size > MAX_FILE_SIZE {
            file_id_to_return += 1;
        }
        return Ok(format!(
            "{file_num}.{ext}",
            file_num = file_id_to_return,
            ext = DATA_FILE_EXT
        ));
    } else {
        return Err("read_dir call failed");
    }
}

fn main() {
    let f = get_file_to_write(
        Path::new("/Users/guido/Downloads/guidcask_data_test/"),
        1024,
    );
    println!("{:?}", f);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        // assert_eq!(get_file_to_write(), 11);
        let path = Path::new("/Users/guido/Downloads/guidcask_data_test/");
        // let my_str = "/Users/guido/Downloads/guidcask_data_test/";
        // let res = get_file_to_write(path, 0);
        // let curr_file = format!(
        //     "{}/{file_num}.{ext}",
        //     // path.to_str().unwrap(),
        //     my_str,
        //     file_num = 42,
        //     ext = DATA_FILE_EXT
        // );

        
        let dir = tempdir().expect("could not create temporary dir");
        let tmp_path = dir.path();
        let res = get_file_to_write(tmp_path, 0);
        println!("output = {:#?}", res);
    }
}
