use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Round {
    red: usize,
    blue: usize,
    green: usize
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    rounds: Vec<Round>,
}


impl Game {
    fn max_s(self: &Game) -> (usize, usize, usize ) {
        let sr = self.rounds.iter().map(|x| x.red).max().unwrap_or(0);
        let sg = self.rounds.iter().map(|x| x.green).max().unwrap_or(0);
        let sb = self.rounds.iter().map(|x| x.blue).max().unwrap_or(0);
        (sr, sg, sb)
    }
}

lazy_static! {
    static ref GR: Regex = Regex::new(r"Game (\d+)").unwrap();
    static ref RR: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
}

pub fn parse(line: &str) -> Option<Game> {

    let lr: Vec<&str> = line.split(":").collect();

    let c = GR.captures(lr[0])?;
    let id_s = c.get(1)?.as_str();
    let id = usize::from_str(id_s).ok()?;

    let gs: Vec<&str> = lr[1].split(";").collect();
    let mut rounds = vec![];
    for g in gs {
        let cols : Vec<&str> = g.split(",").collect();
        let mut r: Round = Round{blue: 0, red: 0, green: 0}; 
        for col in cols {
            let (_, [count, color]) = RR.captures(col).map(|cap| cap.extract())?;
            match color {
                "red" => r.red = usize::from_str(count).ok()?,
                "green" => r.green = usize::from_str(count).ok()?,
                "blue" => r.blue = usize::from_str(count).ok()?,
                _ => {}
            }
        }
        rounds.push(r)
    }
    Some(Game{id, rounds})
}

pub fn run(input: &str) -> usize {
    let lines = input.lines().map(|x| parse(x));

    return lines
        .flatten()
        .filter(|g| {
            let (red, green, blue) = g.max_s();
            let success = red <= 12 && green <= 13 && blue <= 14;
            //println!("{success}: {:?}", g);
            success
        })
        .map(|g| g.id)
        .sum();
}
