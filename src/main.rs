extern crate checksum;
use std::fs;
use checksum::crc::Crc as crc;
use std::os::unix::fs::MetadataExt;
use std::collections::HashMap;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref MAP: HashMap<u64, String> = {
        let mut map = HashMap::new();
        map.insert(999, "xxx".to_owned());
        map.insert(888, "zzz".to_owned());
        map
    };
}

fn main() {
    let dir = std::env::args().nth(1).expect("no dir given");
    let mut file_info: HashMap<u64, Vec<String>> = HashMap::new();
    read_dir(dir.as_str(), &mut file_info);

    println!("{:?}", file_info);
}

fn read_dir(dir: &str, file_info: &mut HashMap<u64, Vec<String>>) {
    let paths = fs::read_dir(dir).unwrap();

    for entry in paths {
        let path = entry.unwrap().path();
        let attr = fs::symlink_metadata(&path).unwrap();
        if attr.is_file() {
            let metadata = fs::metadata(&path).unwrap();
            let mut crc = crc::new(path.to_str().unwrap());
            let filename = path.to_str().unwrap().to_string();
            let checksum = crc.checksum().unwrap().crc64;
            let inode = metadata.ino();
            //file_info.insert(crc.checksum().unwrap().crc64, filename);
            if !file_info.contains_key(&checksum) {
                file_info.insert(checksum, vec![inode.to_string(), filename]);
            } else {
                let inode_str = &file_info.get_mut(&checksum).unwrap()[0];
                if inode.to_string().eq(inode_str) {
                    println!("found equal inode : {}\t filename : {}", inode_str, filename);
                } else {
                    file_info.get_mut(&checksum).unwrap().push(filename);
                }
            }
            println!("{}\t{}\t{}\t{:X}", path.display(), metadata.ino(), metadata.len(), crc.checksum().unwrap().crc64);
        } else if attr.is_dir() {
            read_dir(path.to_str().unwrap(), file_info);
        }
    }
}
