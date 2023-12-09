use std::fs;
mod solutions;

use solutions::*;

fn main() {
    let d = std::env::current_dir().unwrap();
    println!("{}", d.as_path().display());
    let dir = d.as_path().join("src/input/day3.txt");
    println!("{}", dir.display());
    let res = day3::run_2(fs::read_to_string(dir).unwrap().as_str());
    println!("Calculated day3.2 total as {:?}", res)
}
