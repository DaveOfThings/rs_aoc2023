use std::{cmp::Ordering, collections::HashMap};

use crate::day::{Day, Answer, LineBasedInput};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HAND_RE: Regex = Regex::new("([23456789TJQKA])([23456789TJQKA])([23456789TJQKA])([23456789TJQKA])([23456789TJQKA]) ([\\d]+)").unwrap();
}

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn parse(s: &str) -> Option<Hand> {
        let captures = HAND_RE.captures(s);
        match captures {
            Some(caps) => {
                let mut cards: [Card; 5] = [Card::Two; 5];
                for n in 0..5 {
                    cards[n] = match &caps[n] {
                        "2" => Card::Two,
                        "3" => Card::Three,
                        "4" => Card::Four,
                        "5" => Card::Five,
                        "6" => Card::Six,
                        "7" => Card::Seven,
                        "8" => Card::Eight,
                        "9" => Card::Nine,
                        "T" => Card::Ten,
                        "J" => Card::Jack,
                        "Q" => Card::Queen,
                        "K" => Card::King,
                        "A" => Card::Ace,
                        _ => panic!(),
                    }
                }

                let bid = caps[6].parse().unwrap();

                Some(Hand {cards, bid})
            }
            None => {
                None
            }
        }
    }

    fn hand_type(&self) -> HandType {
        let mut per_card: HashMap<Card, usize> = HashMap::new();
        self.cards.iter()
            .map(|c| *per_card.entry(*c).or_insert(0) += 1);
        let mut per_count: [usize; 5] = [0; 5];
        per_card.iter()
            .map(|(_k, v)| per_count[*v] += 1);

        if per_count[5] == 1 {
            HandType::FiveOfAKind
        }
        else if per_count[4] == 1 {
            HandType::FourOfAKind
        }
        else if per_count[3] == 1 && per_count[2] == 1 {
            HandType::FullHouse
        }
        else if per_count[3] == 1 {
            HandType::ThreeOfAKind
        }
        else if per_count[2] == 2 {
            HandType::TwoPair
        }
        else if per_count[2] == 1 {
            HandType::OnePair
        }
        else {
            HandType::HighCard
        }
    }


}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        if self.hand_type() > other.hand_type() {
            Some(Ordering::Greater)
        }
        else if self.hand_type() < other.hand_type() {
            Some(Ordering::Less)
        }
        else {
            // Same type of hand -- card ranking
            let mut result = Ordering::Equal;
            for n in 0..5 {
                if self.cards[n] < other.cards[n] {
                    result = Ordering::Less;
                    break;
                }
                else if self.cards[n] > other.cards[n] {
                    result = Ordering::Greater;
                    break;
                }
            }

            Some(result)
        }
    }
}

pub struct Day7<'a> {
    _input_filename: &'a str,
}



impl<'a> Day for Day7<'a> {
    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

impl<'a> LineBasedInput<Hand> for Day7<'a> {
    fn parse_line(line: &str, _part2: bool) -> Option<Hand> {
        Hand::parse(line)
    }
}

impl<'a> Day7<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }
}

#[cfg(test)]
mod tests {
    
    use crate::{Day, Answer, Day7};

    #[test]
    fn test_input() {
        let d = Day7::new("examples/day7_example1.txt");
        let input = d.process();

        assert_eq!(input.hands.len(), 5);

    }
}