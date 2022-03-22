extern crate checksum;
use std::fs;
use checksum::crc::Crc as crc;
use std::os::unix::fs::MetadataExt;
use std::collections::HashMap;

fn main() {
    let dir = std::env::args().nth(1).expect("no dir given");
    let mut file_info: HashMap<u64, Vec<String>> = HashMap::new();
    let mut dup_files: HashMap<u32, Vec<String>> = HashMap::new();

    read_dir(dir.as_str(), &mut file_info);
    println!("{} kind of file size.", file_info.len());
    filehash_proc(&file_info, &mut dup_files);
    check_dup(&dup_files);
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
            //println!("{:?}", info_vec);
        }
    }
}

fn filehash_proc(file_info: &HashMap<u64, Vec<String>>, dup_files: &mut HashMap<u32, Vec<String>>) {
    for (_inode, info_vec) in &*file_info {
        if info_vec.len() > 1 {
            for (_index, path) in info_vec.iter().enumerate() {
                let metadata = fs::metadata(&path).unwrap();
                let inode = metadata.ino();
                let mut crc = crc::new(path);
                let checksum = crc.checksum().unwrap().crc32;
                if dup_files.contains_key(&checksum) {
                    let inode_str = &dup_files.get_mut(&checksum).unwrap()[0];
                    if !inode.to_string().eq(inode_str) {
                        dup_files.get_mut(&checksum).unwrap().push(path.to_string());
                    }
                } else {
                    dup_files.insert(checksum, vec![inode.to_string(), path.to_string()]);
                }
            }
        } else {
            println!("no dup size {:?}", info_vec);
        }
    }
}

fn read_dir(dir: &str, file_info: &mut HashMap<u64, Vec<String>>) {
    let paths = fs::read_dir(dir).unwrap();

    'outer: for entry in paths {
        let path = entry.unwrap().path();
        let attr = fs::symlink_metadata(&path).unwrap();
        if attr.is_file() {
            let metadata = fs::metadata(&path).unwrap();
            if metadata.len() < 1 {
                continue 'outer;
            }
            //let mut crc = crc::new(path.to_str().unwrap());
            let filename = path.to_str().unwrap().to_string();
            //let checksum = crc.checksum().unwrap().crc64;
            let length = metadata.len();
            if !file_info.contains_key(&length) {
                //file_info.insert(length, vec![inode.to_string(), filename]);
                file_info.insert(length, vec![filename]);
            } else {
                //let inode_str = &file_info.get_mut(&length).unwrap()[0];
                //if inode.to_string().eq(inode_str) {
                    //println!("found equal inode : {}\t filename : {}", inode_str, filename);
                //} else {
                    file_info.get_mut(&length).unwrap().push(filename);
                //}
            }
            //println!("{}\t{}\t{}\t{:X}", path.display(), metadata.ino(), metadata.len(), crc.checksum().unwrap().crc64);
        } else if attr.is_dir() {
            read_dir(path.to_str().unwrap(), file_info);
        }
    }
}
