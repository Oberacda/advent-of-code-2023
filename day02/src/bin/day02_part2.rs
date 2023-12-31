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
    rounds: Vec<Round>,
}

fn parse_input(inputs: Vec<String>) -> Vec<Game> {
    let red_re = Regex::new(r"(?P<red>\d+)\sred").unwrap();
    let green_re = Regex::new(r"(?P<green>\d+)\sgreen").unwrap();
    let blue_re = Regex::new(r"(?P<blue>\d+)\sblue").unwrap();

    let result: Vec<Game> = inputs
        .iter()
        .map(|input| {
            let (_, rounds_str) = input.split_once(':').unwrap();

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
                rounds,
            }
        })
        .collect();
    result
}

fn get_result(games: Vec<Game>) -> u64 {
    let mut result: u64 = 0;
    for game in games {
        let max_red = game.rounds.iter().map(|x| x.red).max().unwrap();
        let max_green = game.rounds.iter().map(|x| x.green).max().unwrap();
        let max_blue = game.rounds.iter().map(|x| x.blue).max().unwrap();
        result += max_red * max_green * max_blue;
    }
    result
}

fn main() {
    let input: Vec<String> = include_str!("../../resources/input.txt")
        .lines()
        .map(str::to_string)
        .collect();

    let games = parse_input(input);
    let res = get_result(games);

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
        let res = get_result(games);
        assert_eq!(res, 2286);
    }
}
