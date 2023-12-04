use crate::day::{Day, Answer};

pub struct Day15<'a> {
    _input_filename: &'a str,
}

impl<'a> Day15<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }
}

impl<'a> Day for Day15<'a> {
    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}
