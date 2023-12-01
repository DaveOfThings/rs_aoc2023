use std::fs::File;
use std::io::Read;
use std::io::{BufReader, BufRead};

use crate::day::{Day, Answer};

pub struct Day1 {
    input_filename: String,
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    pairs: Vec<(usize, usize)>,
}

impl Input {
    // Read input stream, produce Input
    // uses parse_line() (below) to produce a pair of numbers from each input line.
    // Input interpretation varies between part1 and part2 so a bool is passed in
    // to distinguish those cases.
    fn read(input: impl Read, part2: bool) -> Result<Input, String> {
        let mut pairs: Vec<(usize, usize)> = Vec::new();

        let reader = BufReader::new(input);

        for line in reader.lines() {
            let values = Input::parse_line(&line.unwrap(), part2);
            pairs.push(values);
        }
        
        Ok(Input{pairs})
    }

    // Convert one line of input into two numbers, the first and last to appear on
    // the line.  (Input flag, part2, determines which rules are used for
    // recognizing numbers.)
    fn parse_line(line: &str, part2: bool) -> (usize, usize) {
        let matches_p1: [(&str, usize); 10] = [
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ];

        let matches_p2: [(&str, usize); 9] = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ];

        let mut s = String::new();
        let mut saw_first = false;
        let mut first = 0;
        let mut last = 0;
        for c in line.chars() {
            s.push(c);

            for (pattern, value) in matches_p1 {
                if s.ends_with(pattern) {
                    if !saw_first {
                        first = value;
                        saw_first = true;
                    }
                    last = value;
                }
            }

            if part2 {
                for (pattern, value) in matches_p2 {
                    if s.ends_with(pattern) {
                        if !saw_first {
                            first = value;
                            saw_first = true;
                        }
                        last = value;
                    }
                }
            }
        }

        (first, last)
    }
}

// Day1
impl Day1 {
    pub fn new(filename: &str) -> Self {
        Self { input_filename: filename.to_string() }
    }

    // Helper function to open and read the input file
    fn read_input(&self, part2: bool) -> Input {
        let infile = File::open(&self.input_filename)
            .expect(format!("Couldn't open {}", self.input_filename).as_str());

        Input::read(infile, part2)
            .expect(format!("Error parsing {}.", self.input_filename).as_str())
    }

}

impl Day for Day1 {
    // Compute Part 1 solution
    fn part1(&self) -> Answer {
        // Read input file into Input struct, then sum the results.
        let input = self.read_input(false);

        let p1 = input.pairs.iter().map(|(f, l)| f*10+l).sum();

        Answer::Numeric(p1)
    }

    fn part2(&self) -> Answer {
        // Read input file into Input struct, then sum the results.
        // (The diff between part1 and part2 is the flag passed to read_input.  It
        // interprets numbers embedded in lines differently for each part.)
        let input = self.read_input(true);

        let p2 = input.pairs.iter().map(|(f, l)| f*10+l).sum();

        Answer::Numeric(p2)
    }
}

#[cfg(test)]

mod test {
    use std::fs::File;
    use crate::day1::{Day1, Input};
    use crate::day::{Day, Answer};

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let infile = File::open("examples/day1_example1.txt").unwrap();
        let input = Input::read(infile, false).expect("Error parsing.");
        assert_eq!(input.pairs.len(), 4);
        assert_eq!(input.pairs[0], (1, 2));
        assert_eq!(input.pairs[1], (3, 8));
        assert_eq!(input.pairs[2], (1, 5));
        assert_eq!(input.pairs[3], (7, 7));
    }

    #[test]
    // Read part 2 example and confirm inputs
    fn test_read_part2() {
        let infile = File::open("examples/day1_example2.txt").unwrap();
        let input = Input::read(infile, true).expect("Error parsing.");
        assert_eq!(input.pairs.len(), 7);
        assert_eq!(input.pairs[0], (2, 9));
        assert_eq!(input.pairs[1], (8, 3));
        assert_eq!(input.pairs[2], (1, 3));
        assert_eq!(input.pairs[3], (2, 4));
        assert_eq!(input.pairs[4], (4, 2));
        assert_eq!(input.pairs[5], (1, 4));
        assert_eq!(input.pairs[6], (7, 6));
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d: Day1 = Day1::new("examples/day1_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(142));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d: Day1 = Day1::new("examples/day1_example2.txt");
        assert_eq!(d.part2(), Answer::Numeric(281));
    }

    #[test]
    // Compute part 2 result on puzzle input.
    // Despite test_part2 passing, test_part2b failed at first due to an error
    // in numeral recognition.  The string oneight was recognized as one, not
    // eight.
    fn test_part2b() {
        let d: Day1 = Day1::new("data_aoc2023/day1.txt");
        assert_eq!(d.part2(), Answer::Numeric(55686));
    }

    
}
