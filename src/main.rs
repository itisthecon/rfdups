use clap::Parser;
use std::collections::HashMap;

fn main() {
    let args = rfdups::Args::parse();

    let mut file_info: HashMap<u64, Vec<String>> = HashMap::new();
    let mut dup_files: HashMap<String, Vec<String>> = HashMap::new();
    let mut count: u32 = 0;

    for dir in &args.dirs {
        count += rfdups::read_dir(&dir, &mut file_info, count)
    }
    rfdups::filehash_proc(&file_info, &mut dup_files, count);
    rfdups::check_dup(&dup_files, &args);
}
