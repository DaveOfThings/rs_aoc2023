use crate::day::{Day, Answer};

pub struct Day0 {
    _input_filename: String,
}

impl Day0 {
    pub fn new(filename: &str) -> Self {
        Self { _input_filename: filename.to_string() }
    }
}

impl Day for Day0 {
    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}
