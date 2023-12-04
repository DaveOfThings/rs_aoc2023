use crate::day::{Day, Answer};

pub struct Day5<'a> {
    _input_filename: &'a str,
}

impl<'a> Day5<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }
}

impl<'a> Day for Day5<'a> {
    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}
