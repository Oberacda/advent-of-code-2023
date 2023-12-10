use std::collections::{BTreeMap};
use regex::Regex;
use anyhow::Result;
use std::fmt::Display;
use std::error::Error;
use std::fmt;
use rayon::prelude::*;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub enum Move {
    R,
    L
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Self::R),
            'L' => Ok(Self::L),
            _ => Err(())
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Location<'a> {
    left_id: & 'a str,
    right_id: & 'a str
}

#[derive(Debug, Eq, PartialEq)]
pub struct Map <'a> {
    move_sequence: Vec<Move>,
    locations: BTreeMap<& 'a str, Location<'a>>
}

#[derive(Debug, Eq, PartialEq)]
struct ParsingError;

impl Error for ParsingError {}

impl Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse input for puzzle!")
    }
}

pub fn parse_input(input: &str) -> Result<Map> {
    let mapping_re = Regex::new(r"^(?P<id>\w+)\s*=\s*\((?P<left>\w+),\s*(?P<right>\w+)\)$")?;
    let mut lines_iter = input.lines();

    let moves: Vec<Move>= lines_iter.next().ok_or(ParsingError {})?.chars().filter_map(|x| Move::try_from(x).ok()).collect();
    lines_iter.next();
    let mut locations = BTreeMap::new();
    for line in lines_iter {
        let location_line_matches = mapping_re.captures(line).ok_or(ParsingError {})?;
        let id = location_line_matches.name("id").ok_or(ParsingError {})?.as_str();
        let left = location_line_matches.name("left").ok_or(ParsingError {})?.as_str();
        let right = location_line_matches.name("right").ok_or(ParsingError {})?.as_str();

        locations.insert(id, Location {
            left_id: left,
            right_id: right
        });
    }
    Ok(Map {
        move_sequence: moves,
        locations,
    })
}

pub fn get_moves_to_solve(map: &Map) -> Result<u64> {
    let mut moves_count = 0_u64;
    let mut current_element_id = "AAA";
    for map_move in map.move_sequence.iter().cycle() {
        let current_location = map.locations.get(current_element_id).ok_or(ParsingError {})?;
        current_element_id = match map_move {
            Move::R => current_location.right_id,
            Move::L => current_location.left_id
        };
        moves_count += 1;

        if current_element_id.eq("ZZZ") {
            break;
        }
    }
    Ok(moves_count)
}

fn get_moves_from_location(map: &Map, start_location: &str) -> Option<u64> {
    let mut moves_count= 0_u64;
    let mut current_element_id = start_location;
    for map_move in map.move_sequence.iter().cycle() {
        current_element_id = match map.locations.get(current_element_id)
        {
            Some(current_location) => match map_move {
                Move::R => current_location.right_id,
                Move::L => current_location.left_id
            },
            None => return None
        };
        moves_count += 1;

        if current_element_id.ends_with('Z') {
            break;
        }
    }
    Some(moves_count)
}

pub fn get_moves_to_solve_ghost(map: &Map) -> Result<u64> {
    let moves_to_z_location: Vec<u64> = map.locations.par_iter().filter(|(id, _)| id.ends_with('A')).map(| (x, _)| *x).filter_map(|x| get_moves_from_location(&map, x)).collect();
    let moves_count: u64 = moves_to_z_location.par_iter().cloned().reduce(|| 1_u64, |x, y| num::integer::lcm(x, y));
    Ok(moves_count)
}