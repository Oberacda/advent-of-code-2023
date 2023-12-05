use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;

#[derive(Default, Debug,)]
struct Almanac {
    seeds: Vec<u64>,
    seeds_to_soil: HashMap<u64, u64>,
    soil_to_fertilizer: HashMap<u64, u64>,
    fertilizer_to_water: HashMap<u64, u64>,
    water_to_light: HashMap<u64, u64>,
    light_to_temperature: HashMap<u64, u64>,
    temperature_to_humidity: HashMap<u64, u64>,
    humidity_to_location: HashMap<u64, u64>
}

fn create_mapping(map: &mut HashMap<u64, u64>, start_x: u64, start_y: u64, len: u64) {
    (start_x..start_x+len).zip(start_y..start_y+len).for_each(|(from ,to) | {
        map.insert(from, to);
    });
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
            let x = u64::from_str(line_elems.next().unwrap()).unwrap();
            let y = u64::from_str(line_elems.next().unwrap()).unwrap();
            let len = u64::from_str(line_elems.next().unwrap()).unwrap();

            line_mappings.push((x, y, len));
        }
        result.insert(mapping_name, line_mappings);
    }
    result
}

fn parse_mapping(map: &mut HashMap<u64, u64>, encoded_mappings: Vec<(u64, u64, u64)>) {
    for (to, from, len) in encoded_mappings {
        create_mapping(map, from, to, len);
    }
}
fn parse_seeds(seed_config: &str) -> Vec<u64> {
    seed_config.split_terminator(":").nth(1).unwrap().split_ascii_whitespace().map(|x| u64::from_str(x).unwrap()).collect()
}

fn get_location_from_seed(almanac: &Almanac, seed: &u64) -> u64 {
    let soil = almanac.seeds_to_soil.get(seed).unwrap_or(seed);
    let fertilizer = almanac.soil_to_fertilizer.get(soil).unwrap_or(soil);
    let water = almanac.fertilizer_to_water.get(fertilizer).unwrap_or(fertilizer);
    let light = almanac.water_to_light.get(water).unwrap_or(water);
    let temperature = almanac.light_to_temperature.get(light).unwrap_or(light);
    let humidity = almanac.temperature_to_humidity.get(temperature).unwrap_or(temperature);
    let location = almanac.humidity_to_location.get(humidity).unwrap_or(humidity);
    *location
}

fn find_lowest_location(almanac: &Almanac) -> u64 {
    almanac.seeds.iter().map(|x| get_location_from_seed(&almanac, x)).min().unwrap()
}

fn create_almanac(input: Vec<&str>) -> Almanac{
    let mut almanac = Almanac::default();
    let seeds_vec = parse_seeds(input[0]);

    almanac.seeds = seeds_vec;

    let mappings = parse_mapping_lines(&input[1..]);

    for (mapping_name, mapping) in mappings {
        match mapping_name.as_str() {
            "seed-to-soil" => parse_mapping(&mut almanac.seeds_to_soil, mapping),
            "soil-to-fertilizer" => parse_mapping(&mut almanac.soil_to_fertilizer, mapping),
            "fertilizer-to-water" => parse_mapping(&mut almanac.fertilizer_to_water, mapping),
            "water-to-light" => parse_mapping(&mut almanac.water_to_light, mapping),
            "light-to-temperature" => parse_mapping(&mut almanac.light_to_temperature, mapping),
            "temperature-to-humidity" => parse_mapping(&mut almanac.temperature_to_humidity, mapping),
            "humidity-to-location" => parse_mapping(&mut almanac.humidity_to_location, mapping),
            &_ => {eprintln!("Invalid mapping type!")}
        }
    }
    almanac
}

fn main() {
    let now = Instant::now();
    let input: Vec<&str> = include_str!("../resources/input01.txt").split_terminator("\n\n").collect();
    let almanac = create_almanac(input);
    let res = find_lowest_location(&almanac);
    println!("Result: {}", res);
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input01() {

        let input: Vec<&str> = include_str!("../resources/test_input01.txt").split_terminator("\n\n").collect();
        let almanac = create_almanac(input);
        let res = find_lowest_location(&almanac);
        assert_eq!(res, 35);
    }
}