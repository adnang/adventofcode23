use std::str::FromStr;
use regex::{Captures, Regex};

pub fn calc(input: &str) -> Option<usize> {

    fn extract(cap: Captures) -> Option<usize> {
        cap.get(1)
            .map(|t| { usize::from_str(t.as_str()) })?
            .ok()
    }

    let lines = input.lines();
    let r1 = Regex::new(r"[a-z]*(\d).*").unwrap();
    let r2 = Regex::new(r".*(\d)[a-z]*").unwrap();
    let mut nums = vec![];
    for line in lines {
        print!("{}", line);
        let m = r1.captures(line);
        //print!(" {:?}", m);
        let n = r2.captures(line);
        //print!(" {:?}", n);

        let left = m.and_then(extract);
        let right = n.and_then(extract);

        let num: usize = match (left, right) {
            (Some(l), Some(r)) => l * 10 + r,
            _ => 0,
        };
        println!(" {}", num);
        nums.push(num);
    }

    return Some(nums.iter().sum());
}