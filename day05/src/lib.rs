use std::collections::Bound::{Excluded, Included};
use std::collections::{Bound, HashMap};
use std::ops::RangeBounds;
use std::str::FromStr;
use regex::Regex;
use itertools::Itertools;
use rayon::prelude::*;
use ranges::{GenericRange, OperationResult, Ranges};

#[derive(Default, Debug,)]
pub struct Almanac {
    seeds: Vec<(u64, u64)>,
    seeds_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>
}

fn lookup_mapping(map: &[(u64, u64, u64)], value: &u64) -> u64 {
    map.par_iter().find_first(|(from, _, len)| *from <= *value && *value < *from+len).map_or(*value, |(from, to, _)| to + (value - from))
}
fn get_location_from_seed(almanac: &Almanac, seed: &u64) -> u64 {
    let soil = lookup_mapping(&almanac.seeds_to_soil, seed);
    let fertilizer = lookup_mapping(&almanac.soil_to_fertilizer, &soil);
    let water = lookup_mapping(&almanac.fertilizer_to_water, &fertilizer);
    let light = lookup_mapping(&almanac.water_to_light, &water);
    let temperature = lookup_mapping(&almanac.light_to_temperature, &light);
    let humidity = lookup_mapping(&almanac.temperature_to_humidity, &temperature);
    lookup_mapping(&almanac.humidity_to_location, &humidity)
}

pub fn find_lowest_location(almanac: &Almanac) -> u64 {
    almanac.seeds.par_iter().flat_map_iter(|(from, len)| *from..*from+*len).into_par_iter().min_by(|x, y| get_location_from_seed(almanac, x).cmp(&get_location_from_seed(almanac, y))).unwrap()
}

fn range_to_start_and_len(range: &GenericRange<u64>) -> Option<(u64, u64)> {
    let start = match range.start_bound() {
        Included(start) => *start,
        Excluded(start) => *start - 1,
        Bound::Unbounded => {
            return None
        }
    };
    let end = match range.end_bound() {
        Included(end) => *end + 1,
        Excluded(end) => *end,
        Bound::Unbounded => {
            return None
        }
    };
    Some((start, end))
}

fn compress_mapping(mappings: &[(u64, u64, u64)], sources: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut result = Vec::new();
    for (source_start, source_len) in sources {
        let source_range = GenericRange::from(*source_start..*source_start+source_len);
        let overlapping_mappings: Vec<&(u64, u64, u64)> = mappings.iter().filter(|(from, _to, len)| !source_range.is_disjoint(&GenericRange::from(*from..*from+len))).collect();
        let mut covered_ranges = Ranges::new();
        for (map_from, map_to, map_len) in overlapping_mappings {
            let overlapping_range = GenericRange::from(*map_from..*map_from+map_len);
            let intersection_range = match source_range & overlapping_range {
                OperationResult::Empty => {eprintln!("Empty overlapping range!"); continue}
                OperationResult::Single(range) => range,
                OperationResult::Double(_, _) => {eprintln!("Multiple overlap!"); continue}
            };


            if let Some((start, end)) = range_to_start_and_len(&intersection_range) {
                let transformed_start = (start - map_from) + map_to;
                let transformed_len = end - start;

                result.push((transformed_start, transformed_len));
            } else {
                continue;
            }
            covered_ranges.insert(intersection_range);
        }
        let source_ranges = Ranges::from(source_range);
        let uncovered_ranges = source_ranges - covered_ranges;
        for range in uncovered_ranges.as_slice() {

            if let Some((start, end)) = range_to_start_and_len(range) {
                result.push((start, end-start));
            } else {
                continue;
            }
        }
    }
    result
}

pub fn compress_almanac(almanac: &Almanac) -> Vec<(u64, u64)>{
    let soil = compress_mapping(&almanac.seeds_to_soil, &almanac.seeds);
    let fertilizer = compress_mapping(&almanac.soil_to_fertilizer, &soil);
    let water = compress_mapping(&almanac.fertilizer_to_water, &fertilizer);
    let light = compress_mapping(&almanac.water_to_light, &water);
    let temperature = compress_mapping(&almanac.light_to_temperature, &light);
    let humidity = compress_mapping(&almanac.temperature_to_humidity, &temperature);
    compress_mapping(&almanac.humidity_to_location, &humidity)
}
pub fn find_lowest_location_compression(almanac: &Almanac) -> u64 {
    let compressed_mappings = compress_almanac(almanac);
    compressed_mappings.iter().map(|(start, _)| *start).min().unwrap()
}


fn parse_mapping_lines(mapping_lines: &[& str]) -> HashMap<String, Vec<(u64, u64, u64)>> {
    let heading_re = Regex::new(r"^(?P<map_name>[\w-]+)\s+map:$").unwrap();
    let mut result = HashMap::new();
    for mapping in mapping_lines {
        let mut lines = mapping.lines();
        let header = lines.next().unwrap();

        let heading_matches = heading_re.captures(header).unwrap();
        let mapping_name: String = heading_matches["map_name"].to_string();

        let mut line_mappings = Vec::new();
        for line in lines {
            let mut line_elems = line.split_ascii_whitespace();
            let to = u64::from_str(line_elems.next().unwrap()).unwrap();
            let from = u64::from_str(line_elems.next().unwrap()).unwrap();
            let len = u64::from_str(line_elems.next().unwrap()).unwrap();

            line_mappings.push((from, to, len));
        }
        result.insert(mapping_name, line_mappings);
    }
    result
}

pub fn parse_seeds_part1(seed_config: &str) -> Vec<(u64, u64)> {
    seed_config.split_terminator(':').nth(1).unwrap().split_ascii_whitespace().map(|x| (u64::from_str(x).unwrap(), 1)).sorted().collect()
}

pub fn parse_seeds_part2(seed_config: &str) -> Vec<(u64, u64)> {
    seed_config.split_terminator(':').nth(1).unwrap().split_ascii_whitespace().map(|x| u64::from_str(x).unwrap()).tuples().collect()
}



pub fn create_almanac(input: Vec<&str>, parse_seed_config: fn(&str) -> Vec<(u64, u64)>) -> Almanac{
    let mut almanac = Almanac::default();
    let seeds_vec = parse_seed_config(input[0]);

    almanac.seeds = seeds_vec;

    let mappings = parse_mapping_lines(&input[1..]);

    for (mapping_name, mapping) in mappings {
        match mapping_name.as_str() {
            "seed-to-soil" => almanac.seeds_to_soil = mapping,
            "soil-to-fertilizer" => almanac.soil_to_fertilizer = mapping,
            "fertilizer-to-water" => almanac.fertilizer_to_water = mapping,
            "water-to-light" => almanac.water_to_light = mapping,
            "light-to-temperature" => almanac.light_to_temperature = mapping,
            "temperature-to-humidity" => almanac.temperature_to_humidity = mapping,
            "humidity-to-location" => almanac.humidity_to_location = mapping,
            &_ => {eprintln!("Invalid mapping type!")}
        }
    }
    almanac
}
