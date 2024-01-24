use std::{cmp::{max, min}, collections::HashMap, fs::File, io::{BufReader, BufRead}};

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

#[derive(Clone, Copy)]
struct Domain {
    x_min: usize,
    x_max: usize,
    m_min: usize,
    m_max: usize,
    a_min: usize,
    a_max: usize,
    s_min: usize,
    s_max: usize,
}

const EMPTY_DOMAIN: Domain = Domain {
    x_min: 1,
    x_max: 0,
    m_min: 1,
    m_max: 0,
    a_min: 1,
    a_max: 0,
    s_min: 1,
    s_max: 0,
};

impl Domain {
    fn new() -> Domain {
        Domain { 
            x_min: 1, x_max: 4000,
            m_min: 1, m_max: 4000,
            a_min: 1, a_max: 4000,
            s_min: 1, s_max: 4000,
        }
    }

    // split into subdomains for condition false, condition true
    fn split(&self, cond: &Condition) -> (Domain, Domain) {
        let mut f_domain = *self; // copy
        let mut t_domain = *self; // copy

        match cond {
            Condition::True => {
                // f_domain should be empty
                f_domain = EMPTY_DOMAIN;

                // t_domain is unchangeed
            }
            Condition::XGt(value) => {
                // Modify X components
                f_domain.x_max = min(f_domain.x_max, *value);
                t_domain.x_min = max(t_domain.x_min, *value+1);
            }
            Condition::XLt(value) => {
                // Modify X components
                t_domain.x_max = min(f_domain.x_max, *value-1);
                f_domain.x_min = max(t_domain.x_min, *value);
            }
            Condition::MGt(value) => {
                // Modify X components
                f_domain.m_max = min(f_domain.m_max, *value);
                t_domain.m_min = max(t_domain.m_min, *value+1);
            }
            Condition::MLt(value) => {
                // Modify X components
                t_domain.m_max = min(f_domain.m_max, *value-1);
                f_domain.m_min = max(t_domain.m_min, *value);
            }
            Condition::AGt(value) => {
                // Modify X components
                f_domain.a_max = min(f_domain.a_max, *value);
                t_domain.a_min = max(t_domain.a_min, *value+1);
            }
            Condition::ALt(value) => {
                // Modify X components
                t_domain.a_max = min(f_domain.a_max, *value-1);
                f_domain.a_min = max(t_domain.a_min, *value);
            }
            Condition::SGt(value) => {
                // Modify X components
                f_domain.s_max = min(f_domain.s_max, *value);
                t_domain.s_min = max(t_domain.s_min, *value+1);
            }
            Condition::SLt(value) => {
                // Modify X components
                t_domain.s_max = min(f_domain.s_max, *value-1);
                f_domain.s_min = max(t_domain.s_min, *value);
            }
        }

        (f_domain, t_domain)
    }

    fn size(&self) -> usize {
        let x_size = if self.x_max >= self.x_min {
            1 + self.x_max - self.x_min
        }
        else {
            0
        };

        let m_size = if self.m_max >= self.m_min {
            1 + self.m_max - self.m_min
        }
        else {
            0
        };

        let a_size = if self.a_max >= self.a_min {
            1 + self.a_max - self.a_min
        }
        else {
            0
        };

        let s_size = if self.s_max >= self.s_min {
            1 + self.s_max - self.s_min
        }
        else {
            0
        };

        x_size * m_size * a_size * s_size
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

    fn combos_(&self, wf_name: &str, index: usize, domain: Domain) -> usize {
        let wf = self.workflows.get(wf_name).unwrap();
        let (cond, action) = &wf.steps[index];

        let combos = match cond {
            Condition::True => {
                // A
                // No split, just pursue the action
                match &action {
                    Action::Accept => domain.size(),
                    Action::Reject => 0,
                    Action::Continue(next_wf) => {
                        self.combos_(next_wf, 0, domain)
                    }
                }
            }
            _ => {
                // B
                let (f_domain, t_domain) = domain.split(&cond);
                // Evaluate size when condition is true and we take the action
                let t_size = match &action {
                    Action::Accept => t_domain.size(),
                    Action::Reject => 0,
                    Action::Continue(next_wf) => {
                        self.combos_(next_wf, 0, t_domain)
                    }
                };

                // Evaluate size when condition is false and we don't take the action
                let f_size = self.combos_(wf_name, index+1, f_domain);

                t_size + f_size
            }
        };

        combos
    }

    fn combos(&self) -> usize {
        let domain = Domain::new();

        self.combos_("in", 0, domain)
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
        let input = Input::read(self.input_filename);

        Answer::Numeric(input.combos())
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

    #[test]
    fn test_combos() {
        let input = Input::read("examples/day19_example1.txt");

        assert_eq!(input.combos(), 167409079868000);
    }

    #[test]
    fn test_part2() {
        let d = Day19::new("examples/day19_example1.txt");

        assert_eq!(d.part2(), Answer::Numeric(167409079868000));
    }

}
