use std::fs;

fn main() {
    read_dir("./");
}

fn read_dir(dir: &str) {
    let paths = fs::read_dir(dir).unwrap();

    for entry in paths {
        let path = entry.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();
        if metadata.is_file() {
            println!("File: {} Length: {}", path.display(), metadata.len());
        } else if metadata.is_dir() {
            println!("Dir: {}", path.display());
            read_dir(path.to_str().unwrap());
        }
    }
}
