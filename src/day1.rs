use std::collections::HashMap;
use std::str::{FromStr, Lines};
use regex::{Captures, Regex};

pub fn calc(input: &str) -> Option<usize> {

    let lines = input.lines();

    map_lines(lines)
}

fn map_lines(lines: Lines) -> Option<usize> {
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

pub fn calc_2(input: &str) -> Option<usize> {
    let matchers = HashMap::from(
        [("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9")]
    );

    let mut nums = vec![];

    for line in input.lines() {

        let mut matches = HashMap::new();
        for (m, i) in matchers.iter() {
            line
                .find(*m)
                .and_then(|u| {matches.insert(u, *m)});
            line.find(*i)
                .and_then(|u| {matches.insert(u, *i)});
        }

        print!("{line}");

        let get_string_value_at_index = |f: &usize| {
            matches.get(f).and_then(|x| matchers.get(*x).or(Some(x)))
        };

        let min = matches.keys().min();
        let first_num = min
            .and_then(get_string_value_at_index)
            .and_then(|v| {usize::from_str(v).ok()})
            .unwrap_or(0);

        let max = matches.keys().max();
        let last_num = max
            .and_then(get_string_value_at_index)
            .and_then(|v| {usize::from_str(v).ok()})
            .unwrap_or(0);

        println!(" {:?} {first_num}, {:?} {last_num}", min, max);
        println!("{:?}", matches);

        nums.push((10 * first_num) + last_num)
    }

    Some(nums.iter().sum())
}