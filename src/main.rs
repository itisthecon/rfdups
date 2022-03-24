use clap::Parser;
use crc32fast::Hasher;
use crossterm::{cursor::MoveUp, execute, terminal};
use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{stdout, Read};
use std::os::unix::fs::MetadataExt;

#[derive(Parser, Debug)]
#[clap(name = "rfdups")]
#[clap(author = "Tang Jun <ken@gos7.net>")]
#[clap(version)]
#[clap(about = "find duplicate files in directory quickly", long_about = None)]
struct Args {
    #[clap(short('m'), long, help = "summarize dupe information")]
    summarize: bool,

    #[clap(short, long, help = "show size of duplicate files")]
    size: bool,

    #[clap(min_values = 1, multiple_values = true, required = true)]
    dirs: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut file_info: HashMap<u64, Vec<String>> = HashMap::new();
    let mut dup_files: HashMap<String, Vec<String>> = HashMap::new();
    let mut count: u32 = 0;

    for dir in &args.dirs {
        count += read_dir(&dir, &mut file_info, count)
    }
    filehash_proc(&file_info, &mut dup_files, count);
    check_dup(&dup_files, &args);
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

fn clean_up_line() {
    // clean previous out put
    let screen_width = terminal::size().unwrap().0 as usize;
    println!(
        "{}",
        std::iter::repeat(" ")
            .take(screen_width)
            .collect::<String>()
    );
    execute!(stdout(), MoveUp(1)).unwrap();
}

fn check_dup(file_info: &HashMap<String, Vec<String>>, options: &Args) {
    let mut total_size: u64 = 0;
    let mut file_num: u32 = 0;

    clean_up_line();

    for (f_info, fn_vec) in &*file_info {
        if fn_vec.len() > 2 {
            let v: Vec<&str> = f_info.split("_").collect();
            let f_size = v[0].parse::<u64>().unwrap();
            let dup_num = fn_vec.len() - 2;
            total_size += f_size * dup_num as u64;
            file_num += dup_num as u32;

            if options.size {
                println!("{} bytes each:", v[0]);
            }
            for (index, value) in fn_vec.iter().enumerate() {
                if index > 0 {
                    println!("{}", value);
                }
            }
            println!("");
        }
    }

    if options.summarize {
        println!(
            "{} duplicate files, occupying {} bytes",
            file_num, total_size
        );
    }
}

fn filehash_proc(
    file_info: &HashMap<u64, Vec<String>>,
    dup_files: &mut HashMap<String, Vec<String>>,
    mut count: u32,
) {
    let indicator = ['/', '|', '\\', '-'];
    let mut progress: usize = 0;

    for (len, info_vec) in &*file_info {
        if info_vec.len() > 1 {
            for (_index, path) in info_vec.iter().enumerate() {
                let metadata = fs::metadata(&path).unwrap();
                let inode = metadata.ino();
                let key = format!("{}_{}", len, crc32(path));

                if dup_files.contains_key(&key) {
                    let inode_str = &dup_files.get_mut(&key).unwrap()[0];
                    if !inode.to_string().eq(inode_str) {
                        dup_files.get_mut(&key).unwrap().push(path.to_string());
                    }
                } else {
                    dup_files.insert(key, vec![inode.to_string(), path.to_string()]);
                }
            }
        }
        count -= info_vec.len() as u32;
        clean_up_line();
        eprintln!(
            "{}\t{} \tfiles left.",
            indicator[progress],
            count.to_formatted_string(&Locale::en)
        );
        execute!(stdout(), MoveUp(1)).unwrap();
        progress = (progress + 1) % 4;
    }
}

fn read_dir(dir: &str, file_info: &mut HashMap<u64, Vec<String>>, mut count: u32) -> u32 {
    let paths = fs::read_dir(dir).unwrap();
    let indicator = ['-', '\\', '|', '/'];
    let mut progress: usize = 0;

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
            eprintln!(
                "{}\t{} \tfiles found.",
                indicator[progress],
                count.to_formatted_string(&Locale::en)
            );
            execute!(stdout(), MoveUp(1)).unwrap();
            progress = (progress + 1) % 4;
            clean_up_line();
        } else if attr.is_dir() {
            count = read_dir(path.to_str().unwrap(), file_info, count);
        }
    }
    count
}
