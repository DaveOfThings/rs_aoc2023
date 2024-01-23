use std::collections::HashMap;

use crate::day::{Day, Answer};

enum Condition {
    XLt(usize),
    XGt(usize),
    MLt(usize),
    MGt(usize),
    ALt(usize),
    AGt(usize),
    SLt(usize),
    SGt(usize),
}

enum Action {
    Accept,
    Reject,
    Continue(String),
}

impl Action {
    pub fn from_str(s: &str) -> Option<Action> {
        // TODO
    }
}

struct WorkFlow {
    steps: Vec<(Condition, Action)>,
}

impl WorkFlow {
    pub fn from_str(s: &str) -> Option<WorkFlow> {
        // TODO
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    pub fn from_str(s: &str) -> Option<Part> {
        // TODO
    } 
}

struct Input {
    workflows: HashMap<String, WorkFlow>,
    parts: Vec<Part>,
}

impl Input {
    pub fn read(filename: &str) -> Input {
        // TODO
    }
}

pub struct Day19<'a> {
    input_filename: &'a str,
}

impl<'a> Day19<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }
}

impl<'a> Day for Day19<'a> {
    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

#[cfg(test)]
mod test {
    use crate::day19::Input;

    #[test]
    fn test_input() {
        let input = Input::read("examples/day19_example1.txt");
        assert_eq!(input.workflows.len(), 11);
        assert_eq!(input.parts.len(), 5);
    }
}
