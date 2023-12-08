use std::fs;
mod solutions;

use solutions::day2;

fn main() {
    let d = std::env::current_dir().unwrap();
    println!("{}", d.as_path().display());
    let dir = d.as_path().join("src/input/day2.txt");
    println!("{}", dir.display());
    let res = day2::run_2(fs::read_to_string(dir).unwrap().as_str());
    println!("Calculated day2 total as {}", res)
}
