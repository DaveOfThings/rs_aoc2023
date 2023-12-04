use std::io::Read;
use std::io::{BufReader, BufRead};

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum Answer {
    None,
    Numeric(usize),
    String(String),
}

pub trait Day: Sync {

    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

// Read input file, parse each line into a T, then produce a vector of T
pub trait LineBasedInput<T> {
    fn process(&self, input: impl Read, part2: bool) -> Vec<T> {
        let mut records: Vec<T> = Vec::new();
        let reader = BufReader::new(input);

        for line in reader.lines() {
            match line {
                Err(_) => break,
                Ok(line) => {
                    if let Some(record) = Self::parse_line(&line, part2) {
                        records.push(record)    
                    }
                }
            }
        }

        records
    }

    fn parse_line(line: &str, part2: bool) -> Option<T>;
}
