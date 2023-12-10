use std::str::FromStr;
use std::time::Duration;
use rayon::prelude::*;

#[derive(Default, Debug)]
pub struct Race {
    duration: Duration,
    distance_mm: u64
}

pub fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let times_iter = lines[0].split_terminator(':').nth(1).unwrap().split_whitespace().map(|x| u64::from_str(x).unwrap());
    let distances_iter = lines[1].split_terminator(':').nth(1).unwrap().split_whitespace().map(|x| u64::from_str(x).unwrap());
    times_iter.zip(distances_iter).map(|(time, distance)| Race {duration: Duration::from_millis(time), distance_mm: distance}).collect()
}

fn get_race_distance_for_push_duration_mm(duration: &Duration, push_duration: &Duration) -> u64 {
    let race_duration = *duration - *push_duration;
    let speed_mm_ms = push_duration.as_millis() as u64;

    race_duration.as_millis() as u64 * speed_mm_ms
}

pub fn get_winning_race_strategy_count(races: &[Race]) -> u64 {
    races.par_iter().map(|race| {
        let possible_push_durations_ms = race.duration.as_millis() as u64;
        (0..possible_push_durations_ms).into_par_iter().map(|x| get_race_distance_for_push_duration_mm(&race.duration, &Duration::from_millis(x))).filter(|x| *x > race.distance_mm).count() as u64
    }).product()

/*    for race in races.iter() {
        let possible_push_durations_ms = race.duration.as_millis() as u64;
        let winning_strategies: u64 = (0..possible_push_durations_ms).into_par_iter().map(|x| get_race_distance_for_push_duration_mm(&race.duration, &Duration::from_millis(x))).filter(|x| *x > race.distance_mm).count() as u64;
        result *= winning_strategies;
    }
    result
*/}