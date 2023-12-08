use std::collections::HashMap;
use std::str::FromStr;
use regex::{Captures, Regex};

pub fn calc(input: &str) -> Option<usize> {
    let lines= input.lines().map(String::from).collect();

    map_lines(lines)
}

fn map_lines(lines: Vec<String>) -> Option<usize> {
    fn extract(cap: Captures) -> Option<usize> {
        cap.get(1)
            .map(|t| { usize::from_str(t.as_str()) })?
            .ok()
    }

    let r1 = Regex::new(r"[a-z]*(\d).*").unwrap();
    let r2 = Regex::new(r".*(\d)[a-z]*").unwrap();
    let mut nums = vec![];
    for line in lines {
        print!("{}", line);
        let m = r1.captures(line.as_str());
        //print!(" {:?}", m);
        let n = r2.captures(line.as_str());
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

pub fn calc_2(input: &str) -> Option<usize> {
    let matchers = HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine")
    ]);

    let new_in = input.lines().map(|l| {
        let mut r = String::from(l);
        for (k, v) in matchers.iter() {
            r = r.replace(k, v);
        }
        r
    }).collect();

    return map_lines(new_in);
}