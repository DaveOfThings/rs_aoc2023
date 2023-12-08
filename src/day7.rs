use std::{cmp::Ordering, collections::HashMap, fs::File};

use crate::day::{Day, Answer, LineBasedInput};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HAND_RE: Regex = Regex::new("([23456789TJQKA])([23456789TJQKA])([23456789TJQKA])([23456789TJQKA])([23456789TJQKA]) ([\\d]+)").unwrap();
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
    Joker,
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

#[derive(PartialEq, Eq, Ord)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    part2: bool
}

impl Hand {
    fn parse(s: &str, part2: bool) -> Option<Hand> {
        let captures = HAND_RE.captures(s);
        match captures {
            Some(caps) => {
                let mut cards: [Card; 5] = [Card::Two; 5];
                for n in 0..5 {
                    cards[n] = match &caps[n+1] {
                        "2" => Card::Two,
                        "3" => Card::Three,
                        "4" => Card::Four,
                        "5" => Card::Five,
                        "6" => Card::Six,
                        "7" => Card::Seven,
                        "8" => Card::Eight,
                        "9" => Card::Nine,
                        "T" => Card::Ten,
                        "J" => {
                            if part2 { Card::Joker } else {Card::Jack }
                        }
                        "Q" => Card::Queen,
                        "K" => Card::King,
                        "A" => Card::Ace,
                        _ => panic!(),
                    }
                }

                let bid = caps[6].parse().unwrap();

                Some(Hand {cards, bid, part2})
            }
            None => {
                None
            }
        }
    }

    fn hand_type(&self) -> HandType {
        let mut per_card: HashMap<Card, usize> = HashMap::new();
        for c in &self.cards {
            *per_card.entry(*c).or_insert(0) += 1;
        }
        
        // Get number of "multiplicities" (Jokers dont count here.)
        let mut per_count: [usize; 6] = [0; 6];
        for (_k, v) in &per_card {
            if _k != &Card::Joker {
                per_count[*v] += 1; 
            }
        }
        
        let mut hand_type = if per_count[5] == 1 {
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
        };

        // If we have any jokers, promote the hand.
        per_card.entry(Card::Joker).or_insert(0);
        for _n in 0..per_card[&Card::Joker] {
            hand_type = match hand_type {
                HandType::HighCard => HandType::OnePair,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::FullHouse => HandType::FourOfAKind,
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::FiveOfAKind => HandType::FiveOfAKind,
            }
        }

        hand_type
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

struct Input {
    hands: Vec<Hand>,
}

pub struct Day7<'a> {
    input_filename: &'a str,
}



impl<'a> Day for Day7<'a> {
    fn part1(&self) -> Answer {
        let file = File::open(self.input_filename).unwrap();

        let mut input = Input { hands: self.process(file, false) };

        Answer::Numeric(self.winnings(&mut input))
    }

    fn part2(&self) -> Answer {
        let file = File::open(self.input_filename).unwrap();

        let mut input = Input { hands: self.process(file, true) };

        Answer::Numeric(self.winnings(&mut input))
    }
}

impl<'a> LineBasedInput<Hand> for Day7<'a> {
    fn parse_line(line: &str, part2: bool) -> Option<Hand> {
        Hand::parse(line, part2)
    }
}

impl<'a> Day7<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }

    fn winnings(&self, input: &mut Input) -> usize {
        input.hands.sort();

        let mut sum = 0;
        for n in 0..input.hands.len() {
            sum += input.hands[n].bid * (n+1);
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    
    use std::fs::File;

    use crate::{Day, Answer, Day7, day7::LineBasedInput, day7::Input};

    #[test]
    fn test_input() {
        let d = Day7::new("examples/day7_example1.txt");
        let file = File::open(d.input_filename).unwrap();
        let input = Input { hands: d.process(file, false) };

        assert_eq!(input.hands.len(), 5);
        assert_eq!(input.hands[0].bid, 765);
        assert_eq!(input.hands[1].bid, 684);
        assert_eq!(input.hands[2].bid, 28);
        assert_eq!(input.hands[3].bid, 220);
        assert_eq!(input.hands[4].bid, 483);
    }

    #[test]
    fn test_rank() {
        let d = Day7::new("examples/day7_example1.txt");
        let file = File::open(d.input_filename).unwrap();
        let mut input = Input { hands: d.process(file, false) };

        input.hands.sort();

        assert_eq!(input.hands[0].bid, 765);
        assert_eq!(input.hands[1].bid, 220);
        assert_eq!(input.hands[2].bid, 28);
        assert_eq!(input.hands[3].bid, 684);
        assert_eq!(input.hands[4].bid, 483);
    }

    #[test]
    fn test_winnings() {
        let d = Day7::new("examples/day7_example1.txt");
        let file = File::open(d.input_filename).unwrap();

        let mut input = Input { hands: d.process(file, false) };

        assert_eq!(d.winnings(&mut input), 6440);
    }

    #[test]
    fn test_winnings2() {
        let d = Day7::new("examples/day7_example1.txt");
        let file = File::open(d.input_filename).unwrap();

        let mut input = Input { hands: d.process(file, true) };

        assert_eq!(d.winnings(&mut input), 5905);
    }

    #[test]
    fn test_part1() {
        let d = Day7::new("examples/day7_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(6440));
    }

    #[test]
    fn test_part2() {
        let d = Day7::new("examples/day7_example1.txt");

        assert_eq!(d.part2(), Answer::Numeric(5905));
    }
}