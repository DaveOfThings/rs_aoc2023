use std::{fs::File, io::{BufReader, BufRead}};

use crate::day::{Day, Answer};

struct Input {
    sequences: Vec<Vec<i32>>,
}

pub struct Day9<'a> {
    input_filename: &'a str,
}

impl<'a> Day9<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }

    fn read_input(&self) -> Input {
        let infile = File::open(&self.input_filename).expect("Failed to open puzzle input.");
        let mut sequences: Vec<Vec<i32>> = Vec::new();

        let reader = BufReader::new(infile);
        for line in reader.lines() {
            let seq: Vec<i32> = line.unwrap()
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            sequences.push(seq);
        }

        Input { sequences }
    }

    fn next_value(&self, seq: &Vec<i32>) -> i32 {
        let mut diffs: Vec<i32> = Vec::new();
        let mut all_zero = true;
        let last = seq[seq.len()-1];
        for n in 0..seq.len()-1 {
            let diff = seq[n+1] - seq[n];
            diffs.push(diff);
            if diff != 0 {
                all_zero = false;
            }
        }

        if all_zero {
            // All elements of the diff sequence are zero.
            // The next element of seq is equal to seq[LAST] (Which is equal seq[0])
            last
        }
        else {
            // Get the last of the diffs sequence, add it to last of this seq.
            last + self.next_value(&diffs)
        }
    }

    fn prev_value(&self, seq: &Vec<i32>) -> i32 {
        let mut diffs: Vec<i32> = Vec::new();
        let mut all_zero = true;
        let first = seq[0];
        for n in 0..seq.len()-1 {
            let diff = seq[n+1] - seq[n];
            diffs.push(diff);
            if diff != 0 {
                all_zero = false;
            }
        }

        if all_zero {
            // All elements of the diff sequence are zero.
            // The next element of seq is equal to seq[LAST] (Which is equal seq[0])
            first
        }
        else {
            // Get the last of the diffs sequence, add it to last of this seq.
            first - self.prev_value(&diffs)
        }
    }
}

impl<'a> Day for Day9<'a> {
    fn part1(&self) -> Answer {
        let input = self.read_input();

        let sum: i32 = input.sequences.iter()
            .map(|s| self.next_value(s))
            .sum();

        Answer::Numeric(sum as usize)
    }

    fn part2(&self) -> Answer {
        let input = self.read_input();

        let sum: i32 = input.sequences.iter()
            .map(|s| self.prev_value(s))
            .sum();

        Answer::Numeric(sum as usize)
    }
}

#[cfg(test)]
mod test {
    use crate::{day9::Day9, day::{Day, Answer}};

    #[test]
    fn test_input() {
        let d = Day9::new("examples/day9_example1.txt");
        let input = d.read_input();
        assert_eq!(input.sequences.len(), 3);
        assert_eq!(input.sequences[0].len(), 6);
        assert_eq!(input.sequences[1].len(), 6);
        assert_eq!(input.sequences[2].len(), 6);
    }

    #[test]
    fn test_next() {
        let d = Day9::new("examples/day9_example1.txt");
        let input = d.read_input();

        assert_eq!(d.next_value(&input.sequences[0]), 18);
        assert_eq!(d.next_value(&input.sequences[1]), 28);
        assert_eq!(d.next_value(&input.sequences[2]), 68);
    }

    #[test]
    fn test_p1() {
        let d = Day9::new("examples/day9_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(114));


    }

    #[test]
    fn test_prev() {
        let d = Day9::new("examples/day9_example1.txt");
        let input = d.read_input();

        assert_eq!(d.prev_value(&input.sequences[0]), -3);
        assert_eq!(d.prev_value(&input.sequences[1]), 0);
        assert_eq!(d.prev_value(&input.sequences[2]), 5);
    }

    #[test]
    fn test_p2() {
        let d = Day9::new("examples/day9_example1.txt");
        assert_eq!(d.part2(), Answer::Numeric(2));
    }
}