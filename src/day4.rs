use std::fs::File;

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer, LineBasedInput};
struct Card {
    winners: Vec<usize>,
    have: Vec<usize>,
}

lazy_static! {
    static ref CARD_RE: Regex = Regex::new("Card ([ 0-9]+): ([0-9 ]+) \\| ([0-9 ]+)").unwrap();
    static ref NUMBER_RE: Regex = Regex::new("([0-9]+)").unwrap();
}

impl Card {
    fn new(s: &str) -> Card {
        // println!("Card::new for '{}'", s);

        let captures = CARD_RE.captures(s).unwrap();

        // println!("Card no: {}", &captures[1]);
        // println!("Winners: {:?}", &captures[2]);
        // println!("Have: {:?}", &captures[3]);

        let winners = NUMBER_RE.captures_iter(&captures[2]).map(|s| s[0].parse().unwrap()).collect();
        let have = NUMBER_RE.captures_iter(&captures[3]).map(|s| s[0].parse().unwrap()).collect();

        // println!("Done.\n");  
        Card {winners, have}
    }

    fn matches(&self) -> usize {
        self.have.iter().filter(|a| self.winners.contains(a)).count()
    }

    fn value(&self) -> usize {
        let matches = self.matches();
        if matches > 0 {
            1 << (matches-1)
        }
        else {
            0
        }
    }
}

struct Input {
    cards: Vec<Card>,
}

pub struct Day4<'a> {
    input_filename: &'a str,
}

impl<'a> Day4<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }

    fn read_input(&self, _part2: bool) -> Input {
        let infile = File::open(&self.input_filename).expect("Failed to open puzzle input.");
        
        let cards = self.process(infile, false);

        Input {cards}
    }

    fn total_value(&self, input: &Input) -> usize {
        input.cards.iter().map(|c| c.value()).sum()
    }

    fn scratchcards(&self, input: &Input) -> usize {

        // Initialize our card counts with one of each
        let mut count: Vec<usize> = vec![1; input.cards.len()];

        // go through the cards, doing the thing
        for index in 0..input.cards.len() {
            // compute the number of matches for this card.
            let matches = input.cards[index].matches();
            let increment = count[index];

            for offset in 1..=matches {
                count[index+offset] += increment;
            }
        }

        count.iter().sum()
    }
}

impl<'a> Day for Day4<'a> {
    fn part1(&self) -> Answer {
        let input = self.read_input(false);
        Answer::Numeric(self.total_value(&input))

    }

    fn part2(&self) -> Answer {
        let input = self.read_input(true);
        Answer::Numeric(self.scratchcards(&input))
    }
}

impl<'a> LineBasedInput<Card> for Day4<'a> {
    fn parse_line(line: &str, _part2: bool) -> Option<Card> {
        Some(Card::new(line))
    }
}

#[cfg(test)]
mod tests {

    use crate::{Day, Answer, Day4};
    // use crate::day3::Record;

    #[test]
    fn test_input_p1() {        
        let d = Day4::new("examples/day4_example1.txt");
        let input = d.read_input(false);

        assert_eq!(input.cards.len(), 6);
        assert_eq!(input.cards[0].winners.len(), 5);
        assert_eq!(input.cards[1].have.len(), 8);
        assert_eq!(input.cards[2].winners[1], 21);
    }

    #[test]
    fn test_value() {
        let d = Day4::new("examples/day4_example1.txt");
        let input = d.read_input(false);

        assert_eq!(d.total_value(&input), 13);
    }

    #[test]
    fn test_part1() {
        let d = Day4::new("examples/day4_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(13));
    }

    #[test]
    fn test_scratchcards() {
        let d = Day4::new("examples/day4_example1.txt");
        let input = d.read_input(false);

        assert_eq!(d.scratchcards(&input), 30);
    }
}
