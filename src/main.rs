use std::fs;

fn main() {
    let dir = std::env::args().nth(1).expect("no dir given");
    read_dir(dir.as_str());
}

fn read_dir(dir: &str) {
    let paths = fs::read_dir(dir).unwrap();

    for entry in paths {
        let path = entry.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();
        let attr = fs::symlink_metadata(&path).unwrap();
        if attr.is_file() {
            println!("File: {} Length: {}", path.display(), metadata.len());
        } else if attr.is_dir() {
            println!("Dir: {}", path.display());
            read_dir(path.to_str().unwrap());
        }
    }
}
