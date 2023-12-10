use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub enum Move {
    R,
    L
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Location {
    left_id: String,
    right_id: String
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    move_sequence: Vec<Move>,
    locations: BTreeMap<String, Location>
}

pub fn parse_input(input: &str) -> Map {
    todo!("Not Implemented!");
}

pub fn get_moves_to_solve(map: &Map) -> u64 {
    todo!{"Not Implemented!"}
}