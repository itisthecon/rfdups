extern crate checksum;
use std::fs;
use checksum::crc::Crc as crc;
use std::os::unix::fs::MetadataExt;
use std::collections::HashMap;

fn main() {
    let dir = std::env::args().nth(1).expect("no dir given");
    read_dir(dir.as_str());
}

fn read_dir(dir: &str) {
    let paths = fs::read_dir(dir).unwrap();

    for entry in paths {
        let path = entry.unwrap().path();
        let attr = fs::symlink_metadata(&path).unwrap();
        if attr.is_file() {
            let metadata = fs::metadata(&path).unwrap();
            let mut crc = crc::new(path.to_str().unwrap());
            println!("{}\t{}\t{}\t{:X}", path.display(), metadata.ino(), metadata.len(), crc.checksum().unwrap().crc64);
        } else if attr.is_dir() {
            read_dir(path.to_str().unwrap());
        }
    }
}
