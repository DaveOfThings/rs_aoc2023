use crate::day::{Day, Answer};

pub struct Day16<'a> {
    _input_filename: &'a str,
}

impl<'a> Day16<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }
}

impl<'a> Day for Day16<'a> {
    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}
