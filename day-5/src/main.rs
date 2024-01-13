use multimap::MultiMap;
use std::{fs::read_to_string, thread};

mod part_one;

use part_one::solution_one::{parse_seeds, MapTransform, parse_map_transform, Seed};

use crate::part_one::solution_one::parse_seeds_part_two;

#[derive(PartialEq, Hash, Eq, Debug, Copy, Clone)]
enum ParserStatus {
    Unknown,
    Seeds,
    Seed2Soil,
    Soil2Fertilizer,
    Fertilizer2Water,
    Water2Light,
    Light2Temperature,
    Temperature2Humidity,
    Humidity2Location
}

fn part_one_solution(seeds: &Vec<u64>, maps : &MultiMap<ParserStatus, MapTransform>) -> u64{
    let v = vec![ParserStatus::Seed2Soil, ParserStatus::Soil2Fertilizer, ParserStatus::Fertilizer2Water, ParserStatus::Water2Light, ParserStatus::Light2Temperature, ParserStatus::Temperature2Humidity, ParserStatus::Humidity2Location];
    let mut finals : u64 = std::u64::MAX;
    for seed in seeds.into_iter() {
        let mut source_val = *seed;
        for st in v.iter() {
            for m in maps.get_vec(&st).unwrap() {
                if m.is_in_source_range(source_val) {
                    source_val = m.transform(source_val);
                    break;
                }
                continue;
            }
            if *st == ParserStatus::Humidity2Location {
                if source_val < finals {
                    finals = source_val;
                }
            }
        }
    }
    println!("PART ONE SOLUTION: {:?}", finals);
    finals
}

fn main() {
    let filename = "input.txt";
    
    let mut status : ParserStatus = ParserStatus::Unknown;
    let mut maps : MultiMap<ParserStatus, MapTransform> = MultiMap::new();
    //
    let mut seeds : Vec<u64> = Vec::new();
    let mut seeds_part_two : Vec<Seed> = Vec::new();

    for line in read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
    {
        if line.starts_with("seeds") {
            status = ParserStatus::Seeds;
        }
        if line.starts_with("seed-to-soil") {
            status = ParserStatus::Seed2Soil;
            continue;
        }
        if line.starts_with("soil-to-fertilizer") {
            status = ParserStatus::Soil2Fertilizer;
            continue;
        }
        if line.starts_with("fertilizer-to-water") {
            status = ParserStatus::Fertilizer2Water;
            continue;
        }
        if line.starts_with("water-to-light") {
            status = ParserStatus::Water2Light;
            continue;
        }
        if line.starts_with("light-to-temperature") {
            status = ParserStatus::Light2Temperature;
            continue;
        }
        if line.starts_with("temperature-to-humidity") {
            status = ParserStatus::Temperature2Humidity;
            continue;
        }
        if line.starts_with("humidity-to-location") {
            status = ParserStatus::Humidity2Location;
            continue;
        }
        if line.trim().len() == 0 {
            status = ParserStatus::Unknown;
            continue;
        }
        match status {
            ParserStatus::Seeds => {
                seeds_part_two = parse_seeds_part_two(line);
                seeds = parse_seeds(line);
            },
            ParserStatus::Seed2Soil => {
                maps.insert(ParserStatus::Seed2Soil, parse_map_transform(line));
            },
            ParserStatus::Soil2Fertilizer => {
                maps.insert(ParserStatus::Soil2Fertilizer, parse_map_transform(line));
            },
            ParserStatus::Fertilizer2Water => {
                maps.insert(ParserStatus::Fertilizer2Water, parse_map_transform(line));
            },
            ParserStatus::Water2Light => {
                maps.insert(ParserStatus::Water2Light, parse_map_transform(line));
            },
            ParserStatus::Light2Temperature => {
                maps.insert(ParserStatus::Light2Temperature, parse_map_transform(line));
            },
            ParserStatus::Temperature2Humidity => {
                maps.insert(ParserStatus::Temperature2Humidity, parse_map_transform(line));
            },
            ParserStatus::Humidity2Location => {
                maps.insert(ParserStatus::Humidity2Location, parse_map_transform(line)); 
            },
            _ => {},
        }
    }
    println!("SEEDS: {:?}", seeds_part_two);
    part_one_solution(&seeds, &maps);
    // PART TWO
    let mut finals : u64 = std::u64::MAX;
    let v = vec![ParserStatus::Seed2Soil, ParserStatus::Soil2Fertilizer, ParserStatus::Fertilizer2Water, ParserStatus::Water2Light, ParserStatus::Light2Temperature, ParserStatus::Temperature2Humidity, ParserStatus::Humidity2Location];
    thread::scope(|s| {
        let mut children = vec![];
    for seed in seeds_part_two {
        let transf = (seed.ini..seed.fin).map(|seed_value| -> u64 {
            let mut source_val = seed_value;
            for st in v.iter() {
                for m in maps.get_vec(st).unwrap() {
                    if m.is_in_source_range(source_val) {
                        source_val = m.transform(source_val);
                        break;
                    }
                    continue;
                }
            }
            source_val
        });
        children.push(
            s.spawn(|| {
                *transf.collect::<Vec<u64>>().iter().min().unwrap()
            })
        );
    }

    for child in children {
        let maybe_final = child.join().unwrap();
        if maybe_final < finals {
            finals = maybe_final;
        }
    }
    });
    println!("PART TWO: {}", finals)
}
