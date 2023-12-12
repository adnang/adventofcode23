use lazy_static::lazy_static;
use regex::Regex;
use std::str::{FromStr, Lines};

#[derive(Debug)]
struct RangeMap {
    dest: usize,
    src: usize,
    rng: usize,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Vec<RangeMap>,
    soil_to_fertilizer: Vec<RangeMap>,
    fertilizer_to_water: Vec<RangeMap>,
    water_to_light: Vec<RangeMap>,
    light_to_temperature: Vec<RangeMap>,
    temperature_to_humidity: Vec<RangeMap>,
    humidity_to_location: Vec<RangeMap>,
}

lazy_static! {
    static ref R_SEEDS: Regex = Regex::new(r"\s(\d+)").unwrap();
    static ref R_RANGEMAP: Regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
}

fn parse(input: &str) -> Almanac {
    // single `lines` iterator to use throughout parsing
    let mut lines = input.lines();
    let seeds_str = lines.next().unwrap();

    let seeds = R_SEEDS
        .captures_iter(seeds_str)
        .flat_map(|x| x.get(1))
        .flat_map(|x| usize::from_str(x.as_str()))
        .collect::<Vec<usize>>();

    let (
        mut seed_to_soil,
        mut soil_to_fertilizer,
        mut fertilizer_to_water,
        mut water_to_light,
        mut light_to_temperature,
        mut temperature_to_humidity,
        mut humidity_to_location,
    ) = (vec![], vec![], vec![], vec![], vec![], vec![], vec![]);

    while let Some(map_header) = lines.next() {
        match map_header {
            "seed-to-soil map:" => {
                seed_to_soil = get_range_map(&mut lines);
            }
            "soil-to-fertilizer map:" => {
                soil_to_fertilizer = get_range_map(&mut lines);
            }
            "fertilizer_to_water map:" => {
                fertilizer_to_water = get_range_map(&mut lines);
            }
            "water-to-light map:" => {
                water_to_light = get_range_map(&mut lines);
            }
            "light-to-temperature map:" => {
                light_to_temperature = get_range_map(&mut lines);
            }
            "temperature-to-humidity map:" => {
                temperature_to_humidity = get_range_map(&mut lines);
            }
            "humidity-to-location map:" => {
                humidity_to_location = get_range_map(&mut lines);
            }
            _ => {}
        }
    }

    return Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };
}

fn get_range_map(lines: &mut Lines<'_>) -> Vec<RangeMap> {
    let mut vec: Vec<RangeMap> = vec![];
    while let Some(line) = lines.next() {
        let extract = R_RANGEMAP.captures(line).map(|cap| cap.extract());
        let Some((_, [dest, src, rng])) = extract else {
            break; // reached end of data lines, exit.
        };
        vec.push(RangeMap {
            dest: usize::from_str(dest).unwrap(),
            src: usize::from_str(src).unwrap(),
            rng: usize::from_str(rng).unwrap(),
        })
    }
    vec
}

pub fn calc(input: &str) -> Option<usize> {
    let parsed = parse(input);
    println!("{:?}", parsed);

    None
}
