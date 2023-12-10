use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use crate::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

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
                    Card::K | Card::Q | Card::J | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            },
            Card::K => {
                match other {
                    Card::A => {Some(Less)},
                    Card::K => {Some(Equal)},
                    Card::Q | Card::J | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::Q => {
                match other {
                    Card::A | Card::K => {Some(Less)},
                    Card::Q => {Some(Equal)},
                    Card::J | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::J => {
                match other {
                    Card::A | Card::K | Card::Q => {Some(Less)},
                    Card::J => {Some(Equal)},
                    Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::T => {
                match other {
                    Card::A | Card::K | Card::Q | Card::J => {Some(Less)},
                    Card::T => {Some(Equal)},
                    Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::Nine => {
                match other {
                    Card::A | Card::K | Card::Q | Card::J | Card::T => {Some(Less)},
                    Card::Nine => {Some(Equal)},
                    Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::Eight => {
                match other {
                    Card::A | Card::K | Card::Q | Card::J | Card::T | Card::Nine=> {Some(Less)},
                    Card::Eight => {Some(Equal)},
                    Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::Seven => {
                match other {
                    Card::A | Card::K | Card::Q | Card::J | Card::T | Card::Nine | Card::Eight => {Some(Less)},
                    Card::Seven => {Some(Equal)},
                    Card::Six | Card::Five | Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::Six => {
                match other {
                    Card::A | Card::K | Card::Q | Card::J | Card::T | Card::Nine | Card::Eight | Card::Seven => {Some(Less)},
                    Card::Six => {Some(Equal)},
                    Card::Five | Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::Five => {
                match other {
                    Card::A | Card::K | Card::Q | Card::J | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six => {Some(Less)},
                    Card::Five => {Some(Equal)},
                    Card::Four | Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::Four => {
                match other {
                    Card::A | Card::K | Card::Q | Card::J | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five => {Some(Less)},
                    Card::Four => {Some(Equal)},
                    Card::Three | Card::Two => {Some(Greater)}
                }
            }
            Card::Three => {
                match other {
                    Card::A | Card::K | Card::Q | Card::J | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four => {Some(Less)},
                    Card::Three => {Some(Equal)},
                    Card::Two => {Some(Greater)}
                }
            }
            Card::Two => {
                match other {
                    Card::A | Card::K | Card::Q | Card::J | Card::T | Card::Nine | Card::Eight | Card::Seven | Card::Six | Card::Five | Card::Four | Card::Three => {Some(Less)},
                    Card::Two => {Some(Equal)},
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
            FiveOfAKind => {
                match other {
                    FiveOfAKind => {Some(Equal)}
                    FourOfAKind | FullHouse | ThreeOfAKind | TwoPair | OnePair | HighCard => {Some(Greater)}
                }
            }
            FourOfAKind => {
                match other {
                    FiveOfAKind => {Some(Less)},
                    FourOfAKind => {Some(Equal)},
                    FullHouse | ThreeOfAKind | TwoPair | OnePair | HighCard => {Some(Greater)}
                }
            }
            FullHouse => {
                match other {
                    FiveOfAKind| FourOfAKind => {Some(Less)},
                    FullHouse => {Some(Equal)},
                    ThreeOfAKind | TwoPair | OnePair | HighCard => {Some(Greater)}
                }
            }
            ThreeOfAKind => {
                match other {
                    FiveOfAKind| FourOfAKind | FullHouse => {Some(Less)},
                    ThreeOfAKind => {Some(Equal)},
                    TwoPair | OnePair | HighCard => {Some(Greater)}
                }
            }
            TwoPair => {
                match other {
                    FiveOfAKind| FourOfAKind | FullHouse | ThreeOfAKind => {Some(Less)},
                    TwoPair => {Some(Equal)},
                    OnePair | HighCard => {Some(Greater)}
                }
            }
            OnePair => {
                match other {
                    FiveOfAKind| FourOfAKind | FullHouse | ThreeOfAKind | TwoPair => {Some(Less)},
                    OnePair => {Some(Equal)},
                    HighCard => {Some(Greater)}
                }
            }
            HighCard => {
                match other {
                    FiveOfAKind| FourOfAKind | FullHouse | ThreeOfAKind | TwoPair | OnePair => {Some(Less)},
                    HighCard => {Some(Equal)},
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

        let highest_number_of_eq_cards = cards_set.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

        if *highest_number_of_eq_cards.1 == 5 {
            return FiveOfAKind;
        }

        if *highest_number_of_eq_cards.1 == 4 {
            return FourOfAKind;
        }

        if *highest_number_of_eq_cards.1 == 3 {
            let two_similar = cards_set.iter().find(|(_, x)| **x == 2);

            return if two_similar.is_some() {
                FullHouse
            } else {
                ThreeOfAKind
            }
        }

        if *highest_number_of_eq_cards.1 == 2 {

            let two_similar_count = cards_set.iter().filter(|(_, x)| **x == 2).count();
            return if two_similar_count == 2 {
                TwoPair
            } else {
                OnePair
            }
        }

        HighCard
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
    let ranks = 1..=hands.len()as u64;
    hands.iter().map(|hand| hand.bid).zip(ranks).map(|(bid, rank)| bid * rank).sum()

}