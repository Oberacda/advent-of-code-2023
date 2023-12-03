use regex::Regex;
use std::str::FromStr;

#[derive(Default)]
struct Round {
    red: u64,
    blue: u64,
    green: u64,
}

#[derive(Default)]
struct Game {
    id: u64,
    rounds: Vec<Round>,
}

fn parse_input(inputs: Vec<String>) -> Vec<Game> {
    let game_re = Regex::new(r"^Game\s(\d+)$").unwrap();
    let red_re = Regex::new(r"(?P<red>\d+)\sred").unwrap();
    let green_re = Regex::new(r"(?P<green>\d+)\sgreen").unwrap();
    let blue_re = Regex::new(r"(?P<blue>\d+)\sblue").unwrap();

    let result: Vec<Game> = inputs
        .iter()
        .map(|input| {
            let (game_id_str, rounds_str) = input.split_once(':').unwrap();
            let game_id = u64::from_str(
                game_re
                    .captures(game_id_str)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str(),
            )
            .unwrap();

            let rounds: Vec<Round> = rounds_str
                .split_terminator(';')
                .map(|x| {
                    let red_cubes: u64 = match red_re.captures(x) {
                        None => 0,
                        Some(captures) => u64::from_str(&captures["red"]).unwrap(),
                    };
                    let blue_cubes: u64 = match blue_re.captures(x) {
                        None => 0,
                        Some(captures) => u64::from_str(&captures["blue"]).unwrap(),
                    };
                    let green_cubes: u64 = match green_re.captures(x) {
                        None => 0,
                        Some(captures) => u64::from_str(&captures["green"]).unwrap(),
                    };

                    Round {
                        red: red_cubes,
                        blue: blue_cubes,
                        green: green_cubes,
                    }
                })
                .collect();

            Game {
                id: game_id,
                rounds,
            }
        })
        .collect();
    result
}

fn check_valid(game: &Game, max_red: u64, max_blue: u64, max_green: u64) -> bool {
    game.rounds
        .iter()
        .all(|round| round.red <= max_red && round.green <= max_green && round.blue <= max_blue)
}

fn main() {
    let input: Vec<String> = include_str!("../../resources/input.txt")
        .lines()
        .map(str::to_string)
        .collect();

    let games = parse_input(input);
    let res = games
        .iter()
        .filter(|&game| check_valid(game, 12, 14, 13))
        .map(|x| x.id)
        .sum::<u64>();

    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input: Vec<String> = include_str!("../../resources/test_input01.txt")
            .lines()
            .map(str::to_string)
            .collect();

        let games = parse_input(input);
        let res = games
            .iter()
            .filter(|&game| check_valid(game, 12, 14, 13))
            .map(|x| x.id)
            .sum::<u64>();
        assert_eq!(res, 8);
    }
}
