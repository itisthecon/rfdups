use std::fs;
use std::os::unix::fs::MetadataExt;

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
            println!("{}\t{}\t{}", path.display(), metadata.ino(), metadata.len());
        } else if attr.is_dir() {
            //println!("Dir: {}", path.display());
            read_dir(path.to_str().unwrap());
        }
    }
}
