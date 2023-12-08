use std::fs;

mod day1;

fn main() {
    let d = std::env::current_dir().unwrap();
    println!("{}", d.as_path().display());
    let dir = d.as_path().join("src/day1.txt");
    println!("{}", dir.display());
    let res = day1::calc(fs::read_to_string(dir).unwrap().as_str());
    println!("Calculated total as {}", res.unwrap_or(0))
}
