mod common;
pub mod day_01;

use std::env;

use common::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Advent of code 2025 {}", args.len() < 3);

    if args.len() < 3 {
        println!("you need to have 2 args number of day and path to file");
        panic!("too few args")
    }

    let day: String = args.get(1).unwrap().to_string();
    let file_path: String = args.get(2).unwrap().to_string();
    
    println!("running day {} with file {}", day, file_path);
    
    let data = read_file(file_path);
    println!("{}",data.len())
}

