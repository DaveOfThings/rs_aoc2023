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
    lines: Vec<String>,
}

impl Input {
    fn read(input: impl Read) -> Result<Input, String> {
        let mut lines: Vec<String> = Vec::new();

        let reader = BufReader::new(input);

        for line in reader.lines() {
            lines.push(line.unwrap().to_string());
        }
        
        Ok(Input{lines})
    }
}



impl Day1 {
    pub fn new(filename: &str) -> Self {
        Self { input_filename: filename.to_string() }
    }

    fn read_input(&self) -> Input {
        let infile = File::open(&self.input_filename)
            .expect(format!("Couldn't open {}", self.input_filename).as_str());

        Input::read(infile)
            .expect(format!("Error parsing {}.", self.input_filename).as_str())
    }

    fn process_input(&self, input: &Input, part2: bool) -> usize {
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

        let mut sum = 0;
        let mut s = String::new();
        for line in &input.lines {
            s.clear();
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

            sum += first*10+last;
        }

        sum
    }

}

impl Day for Day1 {
    

    fn part1(&self) -> Answer {
        let input = self.read_input();

        let p1 = self.process_input(&input, false);

        Answer::Numeric(p1)
    }

    fn part2(&self) -> Answer {
        let input = self.read_input();

        let p2 = self.process_input(&input, true);

        Answer::Numeric(p2)
    }
}

#[cfg(test)]

mod test {
    use std::fs::File;
    use crate::day1::{Day1, Input};
    use crate::day::{Day, Answer};

    #[test]
    fn test_read_normal() {
        let infile = File::open("examples/day1_example1.txt").unwrap();
        let input = Input::read(infile).expect("Error parsing.");
        assert_eq!(input.lines.len(), 4);
        assert_eq!(input.lines[0], "1abc2");
    }

    #[test]
    fn test_part1() {
        // Based on the example in part 1.
        let d: Day1 = Day1::new("examples/day1_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(142));
    }

    #[test]
    fn test_part2() {
        // Based on the example in part 2.
        let d: Day1 = Day1::new("examples/day1_example2.txt");
        assert_eq!(d.part2(), Answer::Numeric(281));
    }

    #[test]
    fn test_part2b() {
        let d: Day1 = Day1::new("data_aoc2023/day1.txt");
        assert_eq!(d.part2(), Answer::Numeric(55686));
    }

    
}
