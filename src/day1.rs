
use std::fs::File;
use std::io::Read;
use std::io::{BufReader, BufRead};


use crate::day::{Day, Answer};

pub struct Day1 {
    input_filename: String,
}

struct DailyInput {
    vv: Vec<Vec<usize>>,
}

impl DailyInput {
    fn read(input: impl Read) -> Result<DailyInput, String> {
        let mut vv: Vec<Vec<usize>> = Vec::new();
        let mut v: Vec<usize> = Vec::new();

        let reader = BufReader::new(input);

        for line in reader.lines() {
            match line.unwrap().parse::<usize>() {
                Ok(value) => {
                    v.push(value);
                }
                Err(_) => {
                    // A line that isn't parseable as an int separates the vectors
                    if v.len() > 0 {
                        vv.push(v);
                        v = Vec::new();
                    }
                }
            }
        }
        
        if v.len() > 0 {
            vv.push(v);
        }

        if vv.len() > 0{
            Ok(DailyInput { vv })
        }
        else {
            Err(String::from("Empty input."))
        }
    }
}



impl Day1 {
    pub fn new(filename: &str) -> Self {
        Self { input_filename: filename.to_string() }
    }
}

impl Day for Day1 {
    fn part1(&self) -> Answer {
        let infile = File::open(&self.input_filename).expect("Couldn't open input file.");

        let input = DailyInput::read(infile).expect("Error parsing Day 1 input.");
        
        Answer::Numeric(input.vv.len())
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

#[cfg(test)]

mod test {
    use std::fs::File;
    use crate::day1::{Day1, DailyInput};
    use crate::day::{Day, Answer};

    #[test]
    fn test_read_normal() {
        let infile = File::open("data_aoc2023/day1.txt").unwrap();
        let input = DailyInput::read(infile).expect("Error parsing.");
        assert_eq!(input.vv.len(), 254);

        let infile = File::open("examples/day1_example1.txt").unwrap();
        let input = DailyInput::read(infile).expect("Error parsing.");
        assert_eq!(input.vv.len(), 5);
    }

    #[test]
    fn test_part1() {
        let d: Day1 = Day1::new("examples/day1_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(5));
    }
}
