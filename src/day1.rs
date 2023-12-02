use std::fs::File;

use crate::day::{Day, Answer, LineBasedInput};

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    pairs: Vec<(usize, usize)>,
}

pub struct Day1 {
    input_filename: String,
}

impl LineBasedInput<(usize, usize)> for Day1 {
    // Convert one line of input into two numbers, the first and last to appear on
    // the line.  (Input flag, part2, determines which rules are used for
    // recognizing numbers.)
    fn parse_line(line: &str, part2: bool) -> Option<(usize, usize)> {
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

        Some((first, last))
    }
}



// Day1
impl Day1 {
    pub fn new(filename: &str) -> Self {
        Self { input_filename: filename.to_string() }
    }

    
    fn read_input(&self, part2: bool) -> Input {
        let infile = File::open(&self.input_filename)
            .expect("Failed to open puzzle input.");

        let pairs = self.process(infile, part2);
        Input { pairs }
    }
}

impl Day for Day1 {

    // Compute Part 1 solution
    fn part1(&self) -> Answer {
        let input = self.read_input(false);

        let p1 = input.pairs.iter().map(|(first, last)| first*10+last).sum();

        Answer::Numeric(p1)
    }

    fn part2(&self) -> Answer {
        // Read input file into Input struct, then sum the results.
   
        // (The diff between part1 and part2 is the flag passed to read_input.  It
        // interprets numbers embedded in lines differently for each part.)
        let input = self.read_input(true);

        let p1 = input.pairs.iter().map(|(first, last)| first*10+last).sum();

        Answer::Numeric(p1)
    }
}

#[cfg(test)]

mod test {

    use crate::day1::Day1;
    use crate::day::{Day, Answer};

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let d = Day1::new("examples/day1_example1.txt");
        let input = d.read_input(false);
                
        assert_eq!(input.pairs.len(), 4);
        assert_eq!(input.pairs[0], (1, 2));
        assert_eq!(input.pairs[1], (3, 8));
        assert_eq!(input.pairs[2], (1, 5));
        assert_eq!(input.pairs[3], (7, 7));
    }

    #[test]
    // Read part 2 example and confirm inputs
    fn test_read_part2() {
        let d = Day1::new("examples/day1_example2.txt");
        let input = d.read_input(true);

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
