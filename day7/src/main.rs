use std::cmp::Ordering;


#[derive(Debug)]
#[derive(Clone)]
enum Hand {
    HighCard { value: char, hand: String },
    OnePair { value: char, hand: String },
    TwoPair { first: char, second: char, hand: String },
    ThreeOfAKind { value: char, hand: String },
    FullHouse { three: char, two: char, hand: String},
    FourOfAKind { value: char, hand: String },
    FiveOfAKind { value: char, hand: String },
    Empty { hand: String },
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Empty { .. } => 0,
            Hand::HighCard { .. } => 1,
            Hand::OnePair { .. } => 2,
            Hand::TwoPair { .. } => 3,
            Hand::ThreeOfAKind { .. } => 4,
            Hand::FullHouse { .. } => 5,
            Hand::FourOfAKind { .. } => 6,
            Hand::FiveOfAKind { .. } => 7,
        }
    }

    fn hand_str (&self) -> String {
        match self {
            Hand::Empty { hand } => hand.to_string(),
            Hand::HighCard { hand, .. } => hand.to_string(),
            Hand::OnePair { hand, .. } => hand.to_string(),
            Hand::TwoPair { hand, .. } => hand.to_string(),
            Hand::ThreeOfAKind { hand, .. } => hand.to_string(),
            Hand::FullHouse { hand, .. } => hand.to_string(),
            Hand::FourOfAKind { hand, .. } => hand.to_string(),
            Hand::FiveOfAKind { hand, .. } => hand.to_string(),
        }
    }

    fn boost_with_wildcards(&self) -> Hand {
        let wildcards = self.hand_str().matches('J').count();

        match self {
            Hand::Empty { .. } => {
                return Hand::FiveOfAKind { value: 'A', hand: "JJJJJ".to_string() }
            },
            Hand::HighCard { value , .. } => {
                if wildcards == 1 {
                    return Hand::OnePair { value: *value, hand: self.hand_str() }
                } else if wildcards == 2 {
                    return Hand::ThreeOfAKind {  value: *value, hand: self.hand_str() }
                } else if wildcards == 3 {
                    return Hand::FourOfAKind {  value: *value, hand: self.hand_str() }
                } else if wildcards == 4 {
                    return Hand::FiveOfAKind { value: *value, hand: self.hand_str() }
                }
            },
            Hand::OnePair { value, .. } => {
                if wildcards == 1 {
                    return Hand::ThreeOfAKind { value: *value, hand: self.hand_str() }
                } else if wildcards == 2 {
                    return Hand::FourOfAKind { value: *value, hand: self.hand_str() }
                } else if wildcards == 3 {
                    return Hand::FiveOfAKind { value: *value, hand: self.hand_str() }
                }
            },
            Hand::TwoPair { first, second, .. } => {
                if wildcards == 1 {
                    return Hand::FullHouse { three: *first, two: *second, hand: self.hand_str() }
                }
            },
            Hand::ThreeOfAKind { value , ..} => {
                if wildcards == 1 {
                    return Hand::FourOfAKind { value: *value, hand: self.hand_str() }
                } else if wildcards == 2 {
                    return Hand::FiveOfAKind { value: *value, hand: self.hand_str() }
                }
            },
            Hand::FourOfAKind { value , ..} => {
                if wildcards == 1 {
                    return Hand::FiveOfAKind { value: *value, hand: self.hand_str() }
                }
            },
            _ => {},
        }
        return self.clone();
    }
}

const CARD_VALUE: &[char] = &['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARD_VALUE_PART_TWO: &[char] = &['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

fn parse_hand(hand: String, card_values: &[char]) -> Hand {
    let mut char_vector: Vec<char> = hand.chars().collect();
    char_vector
        .sort_by(|a, b| card_values.iter().position(|&x| x == *a)
        .cmp(&card_values.iter().position(|&y| y == *b)));

    return char_vector.iter().enumerate().fold(Hand::Empty { hand }, |current_hand, (i, next_card)| {

        if next_card == &'J' {
            return current_hand;
        }

        match current_hand {
            Hand::Empty { hand } => Hand::HighCard { value: *next_card, hand },
            Hand::HighCard { value , ref hand } => {
                if value == *next_card {
                    return Hand::OnePair { value: *next_card, hand: hand.to_string() }
                }
                else if let Some(previous_value) = char_vector.get(i - 1) {
                    if previous_value == next_card {
                        return Hand::OnePair { value: *next_card, hand: hand.to_string() }
                    }
                }
                return current_hand;
            },
            Hand::OnePair { value , ref hand} => {
                if value == *next_card {
                    return Hand::ThreeOfAKind{ value, hand: hand.to_string() };
                } else if let Some(previous_value) = char_vector.get(i - 1) {
                    if previous_value == next_card {
                        return Hand::TwoPair { first: value, second: *next_card, hand: hand.to_string() }
                    }
                }

                return current_hand;
            },
            Hand::TwoPair { first, second , ref hand} => {
                if first == *next_card {
                    return Hand::FullHouse { three: first, two: second, hand: hand.to_string() }
                } else if second == *next_card {
                    return Hand::FullHouse { three: second, two: first, hand: hand.to_string() }
                } else {
                    return current_hand;
                }
            },
            Hand::ThreeOfAKind { value , ref hand} => {
                if value == *next_card {
                    return Hand::FourOfAKind { value, hand: hand.to_string() }
                } else if let Some(previous_value) = char_vector.get(i - 1) {
                    if previous_value == next_card {
                        return Hand::FullHouse { three: value, two: *next_card, hand: hand.to_string() }
                    }
                }
                return current_hand;
            },
            Hand::FullHouse { .. } => {
                return current_hand;
            },
            Hand::FourOfAKind { value , ref hand} => {
                if value == *next_card {
                    return Hand::FiveOfAKind { value , hand: hand.to_string() }
                } else {
                    return current_hand;
                }
            },
            Hand::FiveOfAKind { .. } => {
                return current_hand;
            },
        }
    });
}

fn part_one(input: &str) {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let hand_string = parts.next().unwrap().to_string();
            let bet = parts.next().unwrap().parse::<usize>().unwrap();
            let hand = parse_hand(hand_string, CARD_VALUE);
            return (hand, bet);
        })
        .collect::<Vec<(Hand, usize)>>();

        hands.sort_by(|a, b| {
            let hand_a = &a.0;
            let hand_b = &b.0;

            if hand_a.score() != hand_b.score() {
                return hand_a.score().cmp(&hand_b.score());
            } else {
                let hand_a_str = hand_a.hand_str();
                let hand_b_str = hand_b.hand_str();
                let mut index = 0;

                while index < hand_a_str.len() {
                    let a_char = hand_a_str.chars().nth(index).unwrap();
                    let b_char = hand_b_str.chars().nth(index).unwrap();

                    if a_char != b_char {
                        return CARD_VALUE.iter().position(|&x| x == b_char)
                            .cmp(&CARD_VALUE.iter().position(|&y| y == a_char));
                    }

                    index += 1;
                }

                return Ordering::Equal;
            }
        });

    let sum = hands.iter().enumerate().fold(0, |acc, step| {
        return acc + (step.0 + 1 ) * step.1.1
    });

    println!("Part one: {}", sum);
}

fn part_two(input: &str) {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let hand_string = parts.next().unwrap().to_string();
            let bet = parts.next().unwrap().parse::<usize>().unwrap();
            let hand = parse_hand(hand_string, CARD_VALUE_PART_TWO).boost_with_wildcards();
            return (hand, bet);
        })
        .collect::<Vec<(Hand, usize)>>();

        hands.sort_by(|a, b| {
            let hand_a = &a.0;
            let hand_b = &b.0;

            if hand_a.score() != hand_b.score() {
                return hand_a.score().cmp(&hand_b.score());
            } else {
                let hand_a_str = hand_a.hand_str();
                let hand_b_str = hand_b.hand_str();
                let mut index = 0;

                while index < hand_a_str.len() {
                    let a_char = hand_a_str.chars().nth(index).unwrap();
                    let b_char = hand_b_str.chars().nth(index).unwrap();

                    if a_char != b_char {
                        return CARD_VALUE_PART_TWO.iter().position(|&x| x == b_char)
                            .cmp(&CARD_VALUE_PART_TWO.iter().position(|&y| y == a_char));
                    }

                    index += 1;
                }

                return Ordering::Equal;
            }
        });

    let sum = hands.iter().enumerate().fold(0, |acc, step| {
        return acc + (step.0 + 1 ) * step.1.1
    });

    println!("Part one: {}", sum);
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}
