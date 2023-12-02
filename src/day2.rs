use std::fs::File;

use crate::day::{Day, Answer, LineBasedInput};

struct Record {
    a: usize,
    b: usize,
}

struct Input {
    records: Vec<Record>,
}

pub struct Day2 {
    input_filename: String,
}

impl Day2 {
    pub fn new(filename: &str) -> Self {
        Self { input_filename: filename.to_string()}
    }

    fn read_input(&self, part2: bool) -> Input {
        let infile = File::open(&self.input_filename).expect("Failed to open puzzle input.");
        let records = self.process(infile, false);
        
        Input { records }
    }
}

impl LineBasedInput<Record> for Day2 {
    fn parse_line(line: &str, part2: bool) -> Option<Record> {
        // TODO: parse a line, return a record.
        Some(Record {a: 0, b: 0} )
    }
}

impl Day for Day2 {


    fn part1(&self) -> Answer {
        let infile = File::open(&self.input_filename).expect("Failed to open puzzle input.");
        let input = self.process(infile, false);

        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day, Day2};
/*
    #[test]
    fn test_input_p1() {
        let mut d = Day2::new("examples/day2_example1.txt");
        let input = d.read_input(false);

        assert_eq!(input.records.len(), 10);

    }

    #[test]
    fn test_input_p2() {
        let mut d = Day2::new("examples/day2_example1.txt");
        let input = d.read_input(true);

        assert_eq!(input.records.len(), 10);
    }
    */
}
