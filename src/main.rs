extern crate checksum;
use std::fs;
use checksum::crc::Crc as crc;
use std::os::unix::fs::MetadataExt;
use std::collections::HashMap;

fn main() {
    let dir = std::env::args().nth(1).expect("no dir given");
    // for debug
    //read_dir_bench(dir.as_str());

    let mut file_info: HashMap<u64, Vec<String>> = HashMap::new();
    read_dir(dir.as_str(), &mut file_info);

    check_dup(&file_info);
}
/*
fn read_dir_bench(dir: &str) {
    let paths = fs::read_dir(dir).unwrap();

    for entry in paths {
        let path = entry.unwrap().path();
        let attr = fs::symlink_metadata(&path).unwrap();
        if attr.is_file() {
        } else if attr.is_dir() {
            println!("{}", path.display());
            read_dir_bench(path.to_str().unwrap());
        }
    }
}
*/

fn check_dup(file_info: &HashMap<u64, Vec<String>>) {
    for (_inode, info_vec) in &*file_info {
        if info_vec.len() > 2 {
            for (index, value) in info_vec.iter().enumerate() {
                if index > 0 {
                    println!("{}", value);
                }
            }
            println!("");
            //println!("{:?}", info_vec);
        }
    }
}

fn read_dir(dir: &str, file_info: &mut HashMap<u64, Vec<String>>) {
    let paths = fs::read_dir(dir).unwrap();

    for entry in paths {
        let path = entry.unwrap().path();
        let attr = fs::symlink_metadata(&path).unwrap();
        if attr.is_file() {
            let metadata = fs::metadata(&path).unwrap();
            if metadata.len() < 1 {
                return
            }
            let mut crc = crc::new(path.to_str().unwrap());
            let filename = path.to_str().unwrap().to_string();
            let checksum = crc.checksum().unwrap().crc64;
            let inode = metadata.ino();
            if !file_info.contains_key(&checksum) {
                file_info.insert(checksum, vec![inode.to_string(), filename]);
            } else {
                let inode_str = &file_info.get_mut(&checksum).unwrap()[0];
                if inode.to_string().eq(inode_str) {
                    //println!("found equal inode : {}\t filename : {}", inode_str, filename);
                } else {
                    file_info.get_mut(&checksum).unwrap().push(filename);
                }
            }
            //println!("{}\t{}\t{}\t{:X}", path.display(), metadata.ino(), metadata.len(), crc.checksum().unwrap().crc64);
        } else if attr.is_dir() {
            read_dir(path.to_str().unwrap(), file_info);
        }
    }
}
