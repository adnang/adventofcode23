use std::fs;

use adventofcode23::day5;

fn main() {
    let d = std::env::current_dir().unwrap();
    println!("{}", d.as_path().display());
    let dir = d.as_path().join("src/day5/input_test.txt");
    println!("{}", dir.display());
    let res = day5::calc(fs::read_to_string(dir).unwrap().as_str());
    println!("Calculated day5 total as {:?}", res)
}
