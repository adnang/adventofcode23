use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Part {
    raw: Box<str>,
    pos: Position,
}


impl Part {

    pub fn intersects(self: &Self, point: &Position) -> bool {
        let x1 = self.pos.x;
        let x2 = self.pos.x + self.raw.len() - 1;
        let y = self.pos.y;

        // println!("Comparing {:?} {x1}-{x2}/{y} with {:?}", self.raw , point);
        if y.abs_diff(point.y) > 1 { return false; }
        if x1.abs_diff(point.x) > 1 && x1.abs_diff(point.x) > 1 && x2.abs_diff(point.x) > 1 { return false; }
        true
    }
}

fn parse(input: &str) -> Option<(Vec<Part>, Vec<Position>)> {
    let r = Regex::new(r"(\d+)").ok()?;
    let s = Regex::new(r"([^0123456789.])").ok()?;

    let mut parts: Vec<Part> = vec![];
    let mut symbols: Vec<Position> = vec![];
    let mut y = 0;
    for line in input.lines() {
        println!("===== {y} =====");
        for cap in r.captures_iter(line) {
            let part_match = cap.get(1).unwrap();
            let raw = Box::from(part_match.as_str());
            let part = Part
            {
                raw,
                pos: Position {
                    x: part_match.start(),
                    y,
                },
            };
            println!("{:?}", part);
            parts.push(part)
        }

        for sym in s.captures_iter(line) {
            let sym_match = sym.get(1)?;
            let ss = sym_match.as_str();
            if !ss.is_empty() && ss != "." {
                let position = Position { x: sym_match.start(), y };
                println!("Sym {:?} - {:?}", ss, position);
                symbols.push(position);
            }
        }

        y += 1;
    }

    Some((parts, symbols))
}

pub fn run(input: &str) -> Option<usize> {
    let (parts, symbols) = parse(input)?;

    println!("========== Finished Parsing =========");
    let mut schematic_parts = vec![];
    for p in parts.to_owned() {
        let mut matched = false;
        for x in symbols.iter().by_ref() {
            if p.intersects(x) {
                schematic_parts.push(usize::from_str(&p.raw).unwrap());
                println!("{:?} intersects with sym {:?}", p, x);
                matched = true;
                break;
            }
        }
        if !matched
        {
            println!("{:?} doesn't intersect with any sym", p)
        }

    }
    println!("====== counted {:?}/{:?} parts", schematic_parts.len(), parts.len());
    println!("====== counted {:?} symbols", symbols.len());


    let map = schematic_parts.iter().sum();
    Some(map)
}