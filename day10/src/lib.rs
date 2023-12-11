use std::collections::{BTreeMap, VecDeque};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use anyhow::Result;
use ndarray::{Array2, Ix2};

use crate::PuzzleError::{LogicError, ParserError};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PuzzleError<'a> {
    ParserError {
        msg: &'a str
    },
    LogicError {
        msg: &'a str
    },
}

impl<'a> Display for PuzzleError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to solve puzzle: {}",
               match self {
                   PuzzleError::ParserError { msg } => msg,
                   PuzzleError::LogicError { msg } => msg
               })
    }
}

impl<'a> Error for PuzzleError<'a> {}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TileTypes {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPose,
}

impl Default for TileTypes {
    fn default() -> Self {
        return TileTypes::Ground;
    }
}

impl TryFrom<char> for TileTypes {
    type Error = PuzzleError<'static>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            'S' => Ok(Self::StartingPose),
            '.' => Ok(Self::Ground),
            _ => Err(Self::Error::ParserError { msg: "Invalid tile char!" })
        }
    }
}

impl Display for TileTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TileTypes::Vertical => { '│' }
            TileTypes::Horizontal => { '─' }
            TileTypes::NorthEast => { '└' }
            TileTypes::NorthWest => { '┘' }
            TileTypes::SouthWest => { '┐' }
            TileTypes::SouthEast => { '┌' }
            TileTypes::Ground => { ' ' }
            TileTypes::StartingPose => { 'S' }
        })
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum EncasementTile {
    Loop,
    Encased,
    Free,
    Occupied
}

impl Default for EncasementTile {
    fn default() -> Self {
        return EncasementTile::Encased;
    }
}

impl Display for EncasementTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            EncasementTile::Encased => 'I',
            EncasementTile::Occupied => 'O',
            EncasementTile::Free => '.',
            EncasementTile::Loop => '+'
        })
    }
}

pub fn parse_input(input: &str) -> Result<Array2<TileTypes>> {
    let row_count = input.lines().count();
    let column_count = input.lines().next().ok_or(ParserError { msg: "Could not get first line!" })?.len();
    let mut output_array = Array2::<TileTypes>::default((row_count, column_count));

    let mut line_index: usize = 0;
    for line in input.lines() {
        let mut char_index: usize = 0;
        for char in line.chars() {
            output_array[[line_index, char_index]] = TileTypes::try_from(char)?;
            char_index += 1;
        }
        line_index += 1;
    }
    Ok(output_array)
}

fn process_neighbor(tiles: &Array2<TileTypes>, nodes_to_process: &mut VecDeque<((usize, usize), TileTypes)>, distance_map: &mut BTreeMap<(usize, usize), u64>, neighbor_index: (usize, usize), new_distance: u64) {
    if !distance_map.contains_key(&neighbor_index) {
        let neighbor_node = tiles[neighbor_index];
        nodes_to_process.push_back((neighbor_index, neighbor_node));
        distance_map.insert(neighbor_index, new_distance);
    } else {
        let old_distance = distance_map.get(&neighbor_index).unwrap();
        if *old_distance > new_distance {
            distance_map.insert(neighbor_index, new_distance);
        }
    }
}

fn get_northern_node(y: usize, x: usize) -> Option<(usize, usize)> {
    let new_y = usize::checked_sub(y, 1)?;
    Some((new_y, x))
}

fn get_southern_node(y: usize, x: usize) -> Option<(usize, usize)> {
    let new_y = usize::checked_add(y, 1)?;
    Some((new_y, x))
}

fn get_eastern_node(y: usize, x: usize) -> Option<(usize, usize)> {
    let new_x = usize::checked_add(x, 1)?;
    Some((y, new_x))
}

fn get_western_node(y: usize, x: usize) -> Option<(usize, usize)> {
    let new_x = usize::checked_sub(x, 1)?;
    Some((y, new_x))
}

fn get_distance_map(tiles: &Array2<TileTypes>) -> Result<BTreeMap<(usize, usize), u64>> {
    let ((start_index_y, start_index_x), _) = tiles.indexed_iter().find(|(_, tile)| TileTypes::StartingPose.eq(tile)).ok_or(LogicError { msg: "No starting element found!" })?;
    let mut nodes_to_process: VecDeque<((usize, usize), TileTypes)> = VecDeque::new();
    let mut distance_map: BTreeMap<(usize, usize), u64> = BTreeMap::new();
    distance_map.insert((start_index_y, start_index_x), 0);

    if let Some(north_node) = get_northern_node(start_index_y, start_index_x) {
        let north_tile = tiles[north_node];
        match north_tile {
            TileTypes::Vertical | TileTypes::SouthEast | TileTypes::SouthWest => {
                nodes_to_process.push_back((north_node, north_tile));
                distance_map.insert(north_node, 1);
            }
            TileTypes::Horizontal | TileTypes::NorthEast | TileTypes::NorthWest | TileTypes::Ground | TileTypes::StartingPose => {}
        }
    }

    if let Some(east_node) = get_eastern_node(start_index_y, start_index_x) {
        let east_tile = tiles[east_node];
        match east_tile {
            TileTypes::Horizontal | TileTypes::NorthWest | TileTypes::SouthWest => {
                nodes_to_process.push_back((east_node, east_tile));
                distance_map.insert(east_node, 1);
            }
            TileTypes::Vertical | TileTypes::NorthEast | TileTypes::SouthEast | TileTypes::Ground | TileTypes::StartingPose => {}
        }
    }

    if let Some(south_node) = get_southern_node(start_index_y, start_index_x) {
        let south_tile = tiles[south_node];
        match south_tile {
            TileTypes::Vertical | TileTypes::NorthWest | TileTypes::NorthEast => {
                nodes_to_process.push_back((south_node, south_tile));
                distance_map.insert(south_node, 1);
            }
            TileTypes::Horizontal | TileTypes::SouthWest | TileTypes::SouthEast | TileTypes::Ground | TileTypes::StartingPose => {}
        }
    }

    if let Some(west_node) = get_western_node(start_index_y, start_index_x) {
        let west_tile = tiles[west_node];
        match west_tile {
            TileTypes::Horizontal | TileTypes::NorthEast | TileTypes::SouthEast => {
                nodes_to_process.push_back((west_node, west_tile));
                distance_map.insert(west_node, 1);
            }
            TileTypes::Vertical | TileTypes::NorthWest | TileTypes::SouthWest | TileTypes::Ground | TileTypes::StartingPose => {}
        }
    }
    while !nodes_to_process.is_empty() {
        let ((y, x), tile) = nodes_to_process.pop_front().ok_or(LogicError { msg: "Failed to get next node!" })?;
        let new_distance = distance_map.get(&(y, x)).ok_or(LogicError { msg: "Failed to find distance? How did we get here?" })? + 1;
        match tile {
            TileTypes::Vertical => {
                if let Some(northern_neighbor) = get_northern_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, northern_neighbor, new_distance);
                }
                if let Some(southern_neighbor) = get_southern_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, southern_neighbor, new_distance);
                }
            }
            TileTypes::Horizontal => {
                if let Some(western_neighbor) = get_western_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, western_neighbor, new_distance);
                }
                if let Some(eastern_neighbor) = get_eastern_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, eastern_neighbor, new_distance);
                }
            }
            TileTypes::NorthEast => {
                if let Some(northern_neighbor) = get_northern_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, northern_neighbor, new_distance);
                }
                if let Some(eastern_neighbor) = get_eastern_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, eastern_neighbor, new_distance);
                }
            }
            TileTypes::NorthWest => {
                if let Some(northern_neighbor) = get_northern_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, northern_neighbor, new_distance);
                }
                if let Some(western_neighbor) = get_western_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, western_neighbor, new_distance);
                }
            }
            TileTypes::SouthWest => {
                if let Some(southern_neighbor) = get_southern_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, southern_neighbor, new_distance);
                }
                if let Some(western_neighbor) = get_western_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, western_neighbor, new_distance);
                }
            }
            TileTypes::SouthEast => {
                if let Some(southern_neighbor) = get_southern_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, southern_neighbor, new_distance);
                }
                if let Some(eastern_neighbor) = get_eastern_node(y, x) {
                    process_neighbor(&tiles, &mut nodes_to_process, &mut distance_map, eastern_neighbor, new_distance);
                }
            }
            TileTypes::Ground => {
                continue;
            }
            TileTypes::StartingPose => {
                continue;
            }
        }
    }
    Ok(distance_map)
}

pub fn get_farthest_tile_in_loop_distance(tiles: &Array2<TileTypes>) -> Result<u64> {
    let distance_map = get_distance_map(tiles)?;
    Ok(distance_map.iter().map(|(_, distance)| *distance).max().unwrap_or(0))
}

fn generate_encasement_map(shape: Ix2, distance_map: &BTreeMap<(usize, usize), u64>) -> Result<Array2<EncasementTile>> {
    let mut encasement_map = Array2::default(shape);

    for (&index, _) in distance_map {
        encasement_map[index] = EncasementTile::Loop;
    }

    let min_x = 0_usize;
    let min_y = 0_usize;
    let max_x = shape[1];
    let max_y = shape[0];



    let mut free_cells: VecDeque<(usize, usize)> = VecDeque::new();

    for x in min_x..max_x {
        if encasement_map[[min_y, x]].eq(&EncasementTile::Encased) {
            encasement_map[[min_y, x]] = EncasementTile::Free;
            free_cells.push_back((min_y, x));
        }
        if encasement_map[[max_y - 1, x]].eq(&EncasementTile::Encased) {
            encasement_map[[max_y - 1, x]] = EncasementTile::Free;
            free_cells.push_back((max_y - 1, x));
        }
    }

    for y in min_y..max_y {
        if encasement_map[[y, min_x]].eq(&EncasementTile::Encased) {
            encasement_map[[y, min_x]] = EncasementTile::Free;
            free_cells.push_back((y, min_x));
        }
        if encasement_map[[y, max_x - 1]].eq(&EncasementTile::Encased) {
            encasement_map[[y, max_x - 1]] = EncasementTile::Free;
            free_cells.push_back((y, max_x - 1));
        }
    }

    while !free_cells.is_empty() {
        let (y, x) = free_cells.pop_front().ok_or(LogicError { msg: "Could not get front of queue!" })?;
        if let Some(north_index) = get_northern_node(y, x) {
            if let Some(north_cell) = encasement_map.get(north_index) {
                if EncasementTile::Encased.eq(north_cell) {
                    encasement_map[north_index] = EncasementTile::Free;
                    free_cells.push_back(north_index);
                }
            }
        }
        if let Some(east_index) = get_eastern_node(y, x) {
            if let Some(east_tile) = encasement_map.get(east_index) {
                if EncasementTile::Encased.eq(east_tile) {
                    encasement_map[east_index] = EncasementTile::Free;
                    free_cells.push_back(east_index);
                }
            }
        }
        if let Some(south_index) = get_southern_node(y, x) {
            if let Some(south_tile) = encasement_map.get(south_index) {
                if EncasementTile::Encased.eq(south_tile) {
                    encasement_map[south_index] = EncasementTile::Free;
                    free_cells.push_back(south_index);
                }
            }
        }
        if let Some(west_index) = get_western_node(y, x) {
            if let Some(west_tile) = encasement_map.get(west_index) {
                if EncasementTile::Encased.eq(west_tile) {
                    encasement_map[west_index] = EncasementTile::Free;
                    free_cells.push_back(west_index);
                }
            }
        }
    }


    println!("{}", encasement_map);

    Ok(encasement_map)
}

pub fn get_encased_cells_count(tiles: &Array2<TileTypes>) -> Result<u64> {
    let distance_map = get_distance_map(tiles)?;
    let encasement_map = generate_encasement_map(tiles.raw_dim(), &distance_map)?;
    Ok(encasement_map.iter().filter(|x| EncasementTile::Encased.eq(x)).count() as u64)
}
