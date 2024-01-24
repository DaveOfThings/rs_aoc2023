use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

use crate::day::{Day, Answer};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WORKFLOW_RE: Regex = Regex::new("([a-z]+)\\{(.*)\\}").unwrap();
    static ref COND_RE: Regex = Regex::new("([xmas][<>])([\\d]+)").unwrap();
    static ref PART_RE: Regex = Regex::new("\\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\\}").unwrap();
}

enum Condition {
    True,
    XLt(usize),
    XGt(usize),
    MLt(usize),
    MGt(usize),
    ALt(usize),
    AGt(usize),
    SLt(usize),
    SGt(usize),
}

impl Condition {
    pub fn from_str(s: &str) -> Option<Condition> {
        // We get a string formatted like "s<1351"
        // after RE capture we have cap[1]:"s<", cap[2]:"1351"
        if let Some(caps) = COND_RE.captures(s) {
            let value = caps[2].parse::<usize>().unwrap();
            let cond = match &caps[1] {
                "x<" => Condition::XLt(value),
                "x>" => Condition::XGt(value),
                "m<" => Condition::MLt(value),
                "m>" => Condition::MGt(value),
                "a<" => Condition::ALt(value),
                "a>" => Condition::AGt(value),
                "s<" => Condition::SLt(value),
                "s>" => Condition::SGt(value),
                _ => panic!("Bad condition"),
            };

            Some(cond)
        }
        else {
            None
        }
    }

    fn eval(&self, part: &Part) -> bool {
        match self {
            Condition::True => true,
            Condition::XLt(value) => part.x < *value,
            Condition::XGt(value) => part.x > *value,
            Condition::MLt(value) => part.m < *value,
            Condition::MGt(value) => part.m > *value,
            Condition::ALt(value) => part.a < *value,
            Condition::AGt(value) => part.a > *value,
            Condition::SLt(value) => part.s < *value,
            Condition::SGt(value) => part.s > *value,
        }
    }
}

enum Action {
    Accept,
    Reject,
    Continue(String),
}

impl Action {
    

    pub fn from_str(s: &str) -> Option<Action> {
        // We get a string with just "A", "R", or a label like "px"
        let action = match s {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::Continue(s.to_string()),
        };

        Some(action)
    }
}

struct WorkFlow {
    name: String,
    steps: Vec<(Condition, Action)>,
}

impl WorkFlow {
    pub fn from_str(s: &str) -> Option<WorkFlow> {
        let mut steps: Vec<(Condition, Action)> = Vec::new();

        if let Some(caps) = WORKFLOW_RE.captures(s) {
            let name = &caps[1];
            let steps_str = &caps[2];

            for step in steps_str.split(",") {
                if step.contains(":") {
                    let split: Vec<_> = step.split(":").collect();
                    // println!("Steps split: {:?}", split);
                    let cond = Condition::from_str(split[0]).unwrap();
                    let action = Action::from_str(split[1]).unwrap();
                    steps.push( (cond, action) );
                }
                else {
                    let action = Action::from_str(step).unwrap();
                    steps.push( (Condition::True, action) );
                }
            }
    
            Some(WorkFlow { name: name.to_string(), steps }) 
        }
        else {
            None
        }
        
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
        // We get a string like "{x=2127,m=1623,a=2188,s=1013}"
        if let Some(caps) = PART_RE.captures(s) {
            let x = caps[1].parse().unwrap();
            let m = caps[2].parse().unwrap();
            let a = caps[3].parse().unwrap();
            let s = caps[4].parse().unwrap();
            Some(Part { x, m, a, s })
        }
        else {
            None
        }
    } 

    pub fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

struct Input {
    workflows: HashMap<String, WorkFlow>,
    parts: Vec<Part>,
}

impl Input {
    pub fn read(filename: &str) -> Input {
        let mut workflows: HashMap<String, WorkFlow> = HashMap::new();
        let mut parts: Vec<Part> = Vec::new();
        let mut on_workflows = true;

        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.unwrap();
            if on_workflows {
                if line.len() == 0 {
                    // switch to reading parts
                    on_workflows = false;
                }
                else {
                    // Read a workflow
                    let workflow = WorkFlow::from_str(&line).unwrap();
                    let name = workflow.name.to_string();
                    workflows.insert(name, workflow);
                }
            }
            else {
                // Read a part
                let part = Part::from_str(&line).unwrap();
                parts.push(part);
            }
        }

        Input { workflows, parts }
    }

    fn part_passes(&self, part: &Part) -> bool {
        // Evaluate whether a part passes
        let pass;
        let mut wf = self.workflows.get("in").unwrap();
        let mut index = 0;

        loop {
            // Test condition for current wf, current index
            let (cond, action) = &wf.steps[index];
            if cond.eval(part) {
                // Do this action
                match action {
                    Action::Accept => {
                        pass = true;
                        break;
                    }
                    Action::Reject => {
                        pass = false;
                        break;
                    }
                    Action::Continue(next_wf) => {
                        index = 0;
                        wf = self.workflows.get(next_wf).unwrap();
                    }
                }
            }
            else {
                index += 1;
            }

        }
        
        pass
    }

    fn rating_sum(&self) -> usize {
        self.parts.iter()
            .filter(|part| self.part_passes(part))
            .map(|part| part.rating())
            .sum()
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
        let input = Input::read(self.input_filename);

        Answer::Numeric(input.rating_sum())
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

#[cfg(test)]
mod test {
    use crate::{day19::{Input, Day19}, day::{Answer, Day}};

    #[test]
    fn test_input() {
        let input = Input::read("examples/day19_example1.txt");
        assert_eq!(input.workflows.len(), 11);
        assert_eq!(input.parts.len(), 5);
    }

    #[test]
    fn test_rating() {
        let input = Input::read("examples/day19_example1.txt");

        assert_eq!(input.parts[0].rating(), 7540);
        assert_eq!(input.parts[2].rating(), 4623);
        assert_eq!(input.parts[4].rating(), 6951);
    }

    #[test]
    fn test_pass() {
        let input = Input::read("examples/day19_example1.txt");

        assert_eq!(input.part_passes(&input.parts[0]), true);
        assert_eq!(input.part_passes(&input.parts[1]), false);
        assert_eq!(input.part_passes(&input.parts[2]), true);
        assert_eq!(input.part_passes(&input.parts[3]), false);
        assert_eq!(input.part_passes(&input.parts[4]), true);
    }

    #[test]
    fn test_rating_sum() {
        let input = Input::read("examples/day19_example1.txt");

        assert_eq!(input.rating_sum(), 19114);
    }


    #[test]
    fn test_part1() {
        let d = Day19::new("examples/day19_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(19114));
    }
}
