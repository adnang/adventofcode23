use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    str::{FromStr, Lines},
    vec,
};

#[derive(Debug)]
struct RangeMap {
    dest: usize,
    src: usize,
    rng: usize,
}
#[derive(Debug)]
struct Almanac {
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
    static ref R_SEEDS_2: Regex = Regex::new(r"\s(\d+)\s(\d+)").unwrap();
    static ref R_RANGEMAP: Regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
}

fn parse_seeds_1(seeds_str: &str) -> Vec<usize> {
    let seeds = R_SEEDS
        .captures_iter(seeds_str)
        .flat_map(|x| x.get(1))
        .flat_map(|x| usize::from_str(x.as_str()))
        .collect::<Vec<usize>>();
    seeds
}

fn parse_almanac(mut lines: &mut Lines<'_>) -> Almanac {
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
            "fertilizer-to-water map:" => {
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

fn walk(src: usize, path: &Vec<RangeMap>) -> usize {
    path.iter()
        .filter(|x| x.src <= src && src < (x.src + x.rng))
        .map(|x| x.dest + src - x.src)
        .next()
        .unwrap_or(src)
}

pub fn calc(input: &str) -> Option<usize> {
    let mut lines = input.lines();

    let seeds = parse_seeds_1(lines.next().unwrap());
    let almanac = parse_almanac(&mut lines);
    //println!("{:#?}", parsed);

    let mut res: HashMap<usize, Vec<usize>> = HashMap::new();

    for seed in seeds {
        let soil = walk(seed, &almanac.seed_to_soil);
        let fertilizer = walk(soil, &almanac.soil_to_fertilizer);
        let water = walk(fertilizer, &almanac.fertilizer_to_water);
        let light = walk(water, &almanac.water_to_light);
        let temperature = walk(light, &almanac.light_to_temperature);
        let humidity = walk(temperature, &almanac.temperature_to_humidity);
        let location = walk(humidity, &almanac.humidity_to_location);

        res.insert(
            seed,
            vec![
                soil,
                fertilizer,
                water,
                light,
                temperature,
                humidity,
                location,
            ],
        );
    }

    println!("Computed >>>>");
    // println!("{:#?}", res);

    res.values().flat_map(|vec| vec.last().map(|&u| u)).min()
}

fn parse_seeds_2(seeds_str: &str) -> Vec<(usize, usize)> {
    let seeds = R_SEEDS_2
        .captures_iter(seeds_str)
        .map(|x| (x.get(1).unwrap().as_str(), x.get(2).unwrap().as_str()))
        .map(|(start, count)| {
            (
                usize::from_str(start).unwrap(),
                usize::from_str(count).unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    seeds
}

fn to_hash(maps: &Vec<RangeMap>) -> Box<HashMap<usize, usize>> {
    let mut res = HashMap::new();

    for map in maps {
        for it in 0..map.rng {
            res.insert(map.src + it, map.dest + it);
        }
    }

    Box::from(res)
}

pub fn calc_2(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let seeds = parse_seeds_2(lines.next().unwrap());
    println!("{:#?}", seeds);

    let almanac = parse_almanac(&mut lines);
    println!("{:#?}", almanac);

    println!("Starting walk");

    let seed_to_soil_hash = to_hash(&almanac.seed_to_soil);
    println!("Finished seed_to_soil_hash");
    let soil_to_fertilizer_hash = to_hash(&almanac.soil_to_fertilizer);
    println!("Finished soil_to_fertilizer_hash");
    let fertilizer_to_water_hash = to_hash(&almanac.fertilizer_to_water);
    println!("Finished fertilizer_to_water_hash");
    let water_to_light_hash = to_hash(&almanac.water_to_light);
    println!("Finished water_to_light_hash");
    let light_to_temperature_hash = to_hash(&almanac.light_to_temperature);
    println!("Finished light_to_temperature_hash");
    let temperature_to_humidity_hash = to_hash(&almanac.temperature_to_humidity);
    println!("Finished temperature_to_humidity_hash");
    let humidity_to_location_hash = to_hash(&almanac.humidity_to_location);
    println!("Finished humidity_to_location_hash");
    let mut res = HashMap::new();

    for seed in seeds
        .iter()
        .flat_map(|&(start, count)| (start..start + count))
    {
        let location = seed_to_soil_hash
            .get(&seed)
            .map(|x| soil_to_fertilizer_hash.get(x).unwrap_or(x))
            .map(|x| fertilizer_to_water_hash.get(x).unwrap_or(x))
            .map(|x| water_to_light_hash.get(x).unwrap_or(x))
            .map(|x| light_to_temperature_hash.get(x).unwrap_or(x))
            .map(|x| temperature_to_humidity_hash.get(x).unwrap_or(x))
            .map(|x| humidity_to_location_hash.get(x).unwrap_or(x))
            .unwrap_or(&seed);

        res.insert(seed, *location);
    }

    res.values().min().map(|&u| u)
}
