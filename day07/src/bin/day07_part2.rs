use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => {
                Err(())
            }
        }
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Card::A => {
                match other {
                    Card::A => {Some(Equal)}
                    Card::K | Card::Q | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            },
            Card::K => {
                match other {
                    Card::A => {Some(Less)},
                    Card::K => {Some(Equal)},
                    Card::Q | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            }
            Card::Q => {
                match other {
                    Card::A | Card::K => {Some(Less)},
                    Card::Q => {Some(Equal)},
                    Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            }
            Card::J => {
                match other {
                    Card::A | Card::K | Card::Q | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two => {Some(Less)},
                    Card::J => {Some(Equal)},
                }
            }
            Card::T => {
                match other {
                    Card::A | Card::K | Card::Q => {Some(Less)},
                    Card::T => {Some(Equal)},
                    Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            }
            Card::Nine => {
                match other {
                    Card::A | Card::K | Card::Q |  Card::T => {Some(Less)},
                    Card::Nine => {Some(Equal)},
                    Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            }
            Card::Eight => {
                match other {
                    Card::A | Card::K | Card::Q |  Card::T | Card::Nine=> {Some(Less)},
                    Card::Eight => {Some(Equal)},
                    Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            }
            Card::Seven => {
                match other {
                    Card::A | Card::K | Card::Q |  Card::T | Card::Nine | Card::Eight => {Some(Less)},
                    Card::Seven => {Some(Equal)},
                    Card::Six | Card::Five | Card::Four | Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            }
            Card::Six => {
                match other {
                    Card::A | Card::K | Card::Q | Card::T | Card::Nine | Card::Eight | Card::Seven => {Some(Less)},
                    Card::Six => {Some(Equal)},
                    Card::Five | Card::Four | Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            }
            Card::Five => {
                match other {
                    Card::A | Card::K | Card::Q |  Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six => {Some(Less)},
                    Card::Five => {Some(Equal)},
                    Card::Four | Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            }
            Card::Four => {
                match other {
                    Card::A | Card::K | Card::Q | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five => {Some(Less)},
                    Card::Four => {Some(Equal)},
                    Card::Three | Card::Two | Card::J => {Some(Greater)}
                }
            }
            Card::Three => {
                match other {
                    Card::A | Card::K | Card::Q | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four => {Some(Less)},
                    Card::Three => {Some(Equal)},
                    Card::Two | Card::J=> {Some(Greater)}
                }
            }
            Card::Two => {
                match other {
                    Card::A | Card::K | Card::Q | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three => {Some(Less)},
                    Card::Two => {Some(Equal)},
                    Card::J => {Some(Greater)}
                }
            }
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Card {
    fn from_str(input: &str) -> Vec<Self> {
        input.chars().map(|x| Card::try_from(x).unwrap()).collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u64
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_order = self.hand_type.partial_cmp(&other.hand_type);
        if let Some(Equal) = hand_order {
            Some(self.cards.cmp(&other.cards))
        } else {
            hand_order
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


#[derive(PartialEq, Eq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl PartialOrd<Self> for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            HandType::FiveOfAKind => {
                match other {
                    HandType::FiveOfAKind => {Some(Equal)}
                    HandType::FourOfAKind | HandType::FullHouse | HandType::ThreeOfAKind | HandType::TwoPair | HandType::OnePair | HandType::HighCard => {Some(Greater)}
                }
            }
            HandType::FourOfAKind => {
                match other {
                    HandType::FiveOfAKind => {Some(Less)},
                    HandType::FourOfAKind => {Some(Equal)},
                    HandType::FullHouse | HandType::ThreeOfAKind | HandType::TwoPair | HandType::OnePair | HandType::HighCard => {Some(Greater)}
                }
            }
            HandType::FullHouse => {
                match other {
                    HandType::FiveOfAKind| HandType::FourOfAKind => {Some(Less)},
                    HandType::FullHouse => {Some(Equal)},
                    HandType::ThreeOfAKind | HandType::TwoPair | HandType::OnePair | HandType::HighCard => {Some(Greater)}
                }
            }
            HandType::ThreeOfAKind => {
                match other {
                    HandType::FiveOfAKind| HandType::FourOfAKind | HandType::FullHouse => {Some(Less)},
                    HandType::ThreeOfAKind => {Some(Equal)},
                    HandType::TwoPair | HandType::OnePair | HandType::HighCard => {Some(Greater)}
                }
            }
            HandType::TwoPair => {
                match other {
                    HandType::FiveOfAKind|HandType::FourOfAKind | HandType::FullHouse | HandType::ThreeOfAKind => {Some(Less)},
                    HandType::TwoPair => {Some(Equal)},
                    HandType::OnePair | HandType::HighCard => {Some(Greater)}
                }
            }
            HandType::OnePair => {
                match other {
                    HandType::FiveOfAKind| HandType::FourOfAKind | HandType::FullHouse | HandType::ThreeOfAKind | HandType::TwoPair => {Some(Less)},
                    HandType::OnePair => {Some(Equal)},
                    HandType::HighCard => {Some(Greater)}
                }
            }
            HandType::HighCard => {
                match other {
                    HandType::FiveOfAKind| HandType::FourOfAKind | HandType::FullHouse | HandType::ThreeOfAKind | HandType::TwoPair | HandType::OnePair => {Some(Less)},
                    HandType::HighCard => {Some(Equal)},
                }
            }
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
       self.partial_cmp(other).unwrap()
    }
}

impl HandType {
    fn from_cards(cards: &Vec<Card>) -> Self {
        let mut cards_set: HashMap<Card, u64> = HashMap::new();
        for card in cards {
            *cards_set.entry(*card).or_insert(0) +=1;
        }

        let mut joker_count: u64 = 0;
        if cards_set.contains_key(&Card::J) {
            joker_count = cards_set[&Card::J];
        }
        let highest_number_of_eq_cards = cards_set
            .iter()
            .filter(|(card, _)| (*card).ne(&Card::J))
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap_or((&Card::J, &0_u64));

        if u64::saturating_sub(5, *highest_number_of_eq_cards.1) <= joker_count {
            return HandType::FiveOfAKind;
        }

        if u64::saturating_sub(4, *highest_number_of_eq_cards.1) <= joker_count {
            return HandType::FourOfAKind;
        }

        if u64::saturating_sub(3, *highest_number_of_eq_cards.1) <= joker_count {
            let available_jokers = u64::saturating_sub(joker_count, u64::saturating_sub(3, *highest_number_of_eq_cards.1));
            let two_similar = cards_set
                .iter()
                .filter(|(x, _)| Card::J.ne(x))
                .filter(|(x, _)| x.ne(&highest_number_of_eq_cards.0))
                .find(|(_, x)| u64::saturating_sub(2, **x) <= available_jokers);


            return if two_similar.is_some() {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }

        if u64::saturating_sub(2, *highest_number_of_eq_cards.1) <= joker_count {
            let available_jokers = u64::saturating_sub(joker_count, u64::saturating_sub(2, *highest_number_of_eq_cards.1));
            let other_similar_count = cards_set
                .iter()
                .filter(|(x, _)| Card::J.ne(x))
                .filter(|(x, _)| x.ne(&highest_number_of_eq_cards.0))
                .filter(|(_, x)| u64::saturating_sub(2, **x) <= available_jokers)
                .count();
            return if other_similar_count > 0 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }

        HandType::HighCard
    }
}

pub fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let mut line_iterator = line.split_whitespace();
        let hand_line = line_iterator.next().unwrap();
        let cards = Card::from_str(hand_line);

        let hand_type = HandType::from_cards(&cards);
        let bid = u64::from_str(line_iterator.next().unwrap()).unwrap();
        hands.push(Hand {cards, bid, hand_type})
    }

    hands
}

pub fn calculate_result(hands: &mut Vec<Hand>) -> u64 {
    hands.sort();
    println!("{:?}", hands);
    let ranks = 1..=hands.len()as u64;
    hands.iter().map(|hand| hand.bid).zip(ranks).map(|(bid, rank)| bid * rank).sum()
}

fn main() {
    let input = include_str!("../../resources/input.txt");
    let now = Instant::now();
    let mut hands = parse_input(input);
    let result = calculate_result(&mut hands);
    let elapsed = now.elapsed();
    println!("Result: {}, Elapsed: {:?}", result, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {

        let input = include_str!("../../resources/test_input.txt");
        let mut hands = parse_input(input);
        println!("Hands: {:?}", hands);
        let result = calculate_result(&mut hands);
        assert_eq!(5905, result);
    }

    #[test]
    fn test_fullhouse_joker() {
        let hand = vec![Card::Four, Card::J, Card::Three, Card::Four, Card::Three];
        let result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FullHouse);

        let hand = vec![Card::Four, Card::J, Card::Three, Card::J, Card::Three];
        let result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FourOfAKind);
    }

    #[test]
    fn test_three_of_a_kind_joker() {
        let hand = vec![Card::Four, Card::J, Card::Three, Card::Two, Card::Three];
        let result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::ThreeOfAKind);

        let hand = vec![Card::Four, Card::J, Card::Two, Card::J, Card::Three];
        let result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::ThreeOfAKind);
    }

    #[test]
    fn test_fullhouse_no_joker() {
        let hand = vec![Card::Four, Card::Four, Card::Three, Card::Four, Card::Three];
        let result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FullHouse);
    }

    #[test]
    fn test_four_of_a_kind_joker() {
        let mut hand = vec![Card::Four, Card::Four, Card::J, Card::Four, Card::Three];
        let mut result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FourOfAKind);
        hand = vec![Card::Four, Card::Four, Card::J, Card::J, Card::Three];
        result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FourOfAKind);
        hand = vec![Card::Four, Card::J, Card::J, Card::J, Card::Three];
        result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FourOfAKind);
        hand = vec![Card::J, Card::J, Card::J, Card::J, Card::Three];
        result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FiveOfAKind);
    }

    #[test]
    fn test_five_of_a_kind_joker() {
        let mut hand = vec![Card::Four, Card::Four, Card::J, Card::Four, Card::Four];
        let mut result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FiveOfAKind);
        hand = vec![Card::Four, Card::J, Card::J, Card::Four, Card::Four];
        result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FiveOfAKind);
        hand = vec![Card::Four, Card::J, Card::J, Card::J, Card::Four];
        result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FiveOfAKind);
        hand = vec![Card::Four, Card::J, Card::J, Card::J, Card::J];
        result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FiveOfAKind);
        hand = vec![Card::J, Card::J, Card::J, Card::J, Card::J];
        result = HandType::from_cards(&hand);
        assert_eq!(result, HandType::FiveOfAKind);
    }

   #[test]
   fn test_two_pair_joker() {
      let mut hand = vec![Card::Four, Card::Three, Card::Two, Card::J, Card::Seven];
       let mut result = HandType::from_cards(&hand);
       assert_eq!(result, HandType::OnePair);
       hand = vec![Card::Four, Card::Four, Card::Three, Card::J, Card::Seven];
       result = HandType::from_cards(&hand);
       assert_eq!(result, HandType::ThreeOfAKind);
   }
}
