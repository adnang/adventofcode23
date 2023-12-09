use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Gear {
    sym: Box<str>,
    pos: Position,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Part {
    raw: Box<str>,
    pos: Position,
}


impl Part {
    pub fn get_val(self: &Part) -> usize {
        usize::from_str(self.raw.as_ref()).unwrap()
    }
    pub fn intersects(self: &Self, gear: &Gear) -> bool {
        let x1 = self.pos.x;
        let x2 = self.pos.x + self.raw.len() - 1;
        let y = self.pos.y;

        //println!("Comparing {:?} {x1}-{x2}/{y} with {:?}", self.raw , gear);
        if y.abs_diff(gear.pos.y) > 1 { return false; }
        if x1.abs_diff(gear.pos.x) > 1 && x2.abs_diff(gear.pos.x) > 1 { return false; }
        true
    }
}

fn parse(input: &str) -> Option<(Vec<Part>, Vec<Gear>)> {
    let r = Regex::new(r"(\d+)").ok()?;
    let s = Regex::new(r"([^0123456789.])").ok()?;

    let mut parts: Vec<Part> = vec![];
    let mut symbols: Vec<Gear> = vec![];
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
            let position = Position { x: sym_match.start(), y };
            let g = Gear {sym: Box::from(sym_match.as_str()),  pos: position};
            println!("{:?}", g);
            symbols.push(g);
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

pub fn run_2(input: &str) -> Option<usize> {
    let (parts, gears) = parse(input)?;

    let mut rr = vec![];
    for gear in gears.iter().filter(|&x| &(*x.sym) == "*") {
        let mut pn = vec![];
        for part in parts.iter().by_ref() {
            let inter = part.intersects(&gear);
            if inter {
                pn.push(Box::from(part));
            }
        }
        if pn.len() == 2 {
            let rat = pn.iter().map(|x| x.get_val() ).reduce(|left, right| { left * right});
            rr.push(rat.unwrap())
        }
    }

    println!("found {:?} gears with 2 parts", rr.len());

    Some(rr.iter().sum())
}