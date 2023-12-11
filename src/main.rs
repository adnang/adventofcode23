use std::fs;
mod solutions;

use solutions::*;

fn main() {
    let d = std::env::current_dir().unwrap();
    println!("{}", d.as_path().display());
    let dir = d.as_path().join("src/solutions/day4/input.txt");
    println!("{}", dir.display());
    let res = day4::calc(fs::read_to_string(dir).unwrap().as_str());
    println!("Calculated day4 total as {:?}", res)
}
