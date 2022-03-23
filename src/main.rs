use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::stdout;
use crc32fast::Hasher;
use std::collections::HashMap;
use std::os::unix::fs::MetadataExt;
use crossterm::{
    cursor::MoveUp,
    execute,
};

fn main() {
    let dir = std::env::args().nth(1).expect("no dir given");
    let mut file_info: HashMap<u64, Vec<String>> = HashMap::new();
    let mut dup_files: HashMap<u32, Vec<String>> = HashMap::new();

    let count = read_dir(dir.as_str(), &mut file_info, 0);
    filehash_proc(&file_info, &mut dup_files, count);
    check_dup(&dup_files);

}

fn crc32(filename: &str) -> u32 {
    let mut hasher = Hasher::new();
    const BUFFER_SIZE: usize = 4096;
    let mut buffer = [0; BUFFER_SIZE];
    let mut file = File::open(&filename).unwrap();
    let _ = file.read(&mut buffer);

    hasher.update(&buffer);
    hasher.finalize()
}

fn check_dup(file_info: &HashMap<u32, Vec<String>>) {
    for (_inode, info_vec) in &*file_info {
        if info_vec.len() > 2 {
            for (index, value) in info_vec.iter().enumerate() {
                if index > 0 {
                    println!("{}", value);
                }
            }
            println!("");
        }
    }
}

fn filehash_proc(file_info: &HashMap<u64, Vec<String>>, dup_files: &mut HashMap<u32, Vec<String>>, mut count: u32) {
    for (_inode, info_vec) in &*file_info {
        if info_vec.len() > 1 {
            for (_index, path) in info_vec.iter().enumerate() {
                let metadata = fs::metadata(&path).unwrap();
                let inode = metadata.ino();
                let checksum = crc32(path);
                if dup_files.contains_key(&checksum) {
                    let inode_str = &dup_files.get_mut(&checksum).unwrap()[0];
                    if !inode.to_string().eq(inode_str) {
                        dup_files.get_mut(&checksum).unwrap().push(path.to_string());
                    }
                } else {
                    dup_files.insert(checksum, vec![inode.to_string(), path.to_string()]);
                }
            }
        }
        count -= info_vec.len() as u32;
        eprintln!("{} \tfiles left.", count);
        execute!(stdout(), MoveUp(1)).unwrap();
    }
}

fn read_dir(dir: &str, file_info: &mut HashMap<u64, Vec<String>>, mut count: u32) -> u32 {
    let paths = fs::read_dir(dir).unwrap();

    'outer: for entry in paths {
        let path = entry.unwrap().path();
        let attr = fs::symlink_metadata(&path).unwrap();
        if attr.is_file() {
            let metadata = fs::metadata(&path).unwrap();
            if metadata.len() < 1 {
                continue 'outer;
            }
            let filename = path.to_str().unwrap().to_string();
            let length = metadata.len();
            if !file_info.contains_key(&length) {
                file_info.insert(length, vec![filename]);
            } else {
                file_info.get_mut(&length).unwrap().push(filename);
            }
            count += 1;
            eprintln!("{} \tfiles found.", count);
            execute!(stdout(), MoveUp(1)).unwrap();
        } else if attr.is_dir() {
            count = read_dir(path.to_str().unwrap(), file_info, count);
        }
    }

    count
}
