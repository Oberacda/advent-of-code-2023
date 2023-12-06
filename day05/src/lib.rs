use std::collections::HashMap;
use std::path::Component::ParentDir;
use std::str::FromStr;
use regex::Regex;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Default, Debug,)]
pub struct Almanac {
    seeds: Vec<u64>,
    seeds_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>
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


fn get_seed_for_location(almanac: &Almanac, location: &u64) -> u64 {
    let humidity = reverse_lookup_mapping(&almanac.humidity_to_location, location);
    let temperature = reverse_lookup_mapping(&almanac.temperature_to_humidity, &humidity);
    let light = reverse_lookup_mapping(&almanac.light_to_temperature, &temperature);
    let water = reverse_lookup_mapping(&almanac.water_to_light, &light);
    let fertilizer = reverse_lookup_mapping(&almanac.fertilizer_to_water, &water);
    let soil = reverse_lookup_mapping(&almanac.soil_to_fertilizer, &fertilizer);
    reverse_lookup_mapping(&almanac.seeds_to_soil, &soil)
}

pub fn find_lowest_location(almanac: &Almanac) -> u64 {
    almanac.seeds.par_iter().map(|x| get_location_from_seed(almanac, x)).min().unwrap()
}

pub fn find_lowest_location_reverse_search(almanac: &Almanac) -> u64 {
    let mut locations: Vec<u64> = almanac.humidity_to_location.par_iter().flat_map(|(_, to, len)| *to..*to+*len).collect();

    for location in 0..u64::MAX {
        let seed = get_seed_for_location(almanac, &location);

        if  almanac.seeds.binary_search(&seed).ok().is_some() {
            return location
        } else {
            continue
        }
    }
    0
}
fn lookup_mapping(map: &[(u64, u64, u64)], value: &u64) -> u64 {
    map.par_iter().find_first(|(from, _, len)| *from <= *value && *value < *from+len).map_or(*value, |(from, to, _)| to + (value - from))
}

fn reverse_lookup_mapping(map: &[(u64, u64, u64)], value: &u64) -> u64 {
    map.par_iter().find_first(|(_, to, len)| *to <= *value && *value < *to+len).map_or(*value, |(from, to, _)| from + (value - to))
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

pub fn parse_seeds_part1(seed_config: &str) -> Vec<u64> {
    seed_config.split_terminator(':').nth(1).unwrap().split_ascii_whitespace().map(|x| u64::from_str(x).unwrap()).sorted().collect()
}

pub fn parse_seeds_part2(seed_config: &str) -> Vec<u64> {
    seed_config.split_terminator(':').nth(1).unwrap().split_ascii_whitespace().map(|x| u64::from_str(x).unwrap()).tuples().flat_map(|(from, len)| (from..from+len)).collect()

}


pub fn create_almanac(input: Vec<&str>, parse_seed_config: fn(&str) -> Vec<u64>) -> Almanac{
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
