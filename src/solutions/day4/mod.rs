use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    name: String,
    winning: Vec<usize>,
    mine: Vec<usize>,
}

pub fn calc(input: &str) -> Option<usize> {
    let cards = parse(input);

    let pts = cards
        .iter()
        .map(|card| {
            let w = card
                .mine
                .iter()
                .filter(|&c| card.winning.contains(c))
                .count();
            if w > 0 {
                return 2_usize.pow((w - 1).try_into().unwrap());
            }
            return 0;
        })
        .sum::<usize>();

    Some(pts)
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(":");
            let (card, rest) = (parts.next(), parts.next());
            parts = rest.unwrap().split("|");
            let (winning_str, mine_str) = (parts.next(), parts.next());
            let winning = parse_nums(winning_str.unwrap());
            let mine = parse_nums(mine_str.unwrap());
            return Card {
                name: String::from(card.unwrap()),
                winning,
                mine,
            };
        })
        .collect::<Vec<Card>>()
}

lazy_static! {
    static ref R: Regex = Regex::new(r"(\d+)").unwrap();
}
fn parse_nums(str: &str) -> Vec<usize> {
    R.captures_iter(str)
        .map(|h| usize::from_str(h.get(1).unwrap().as_str()).unwrap())
        .collect::<Vec<usize>>()
}
