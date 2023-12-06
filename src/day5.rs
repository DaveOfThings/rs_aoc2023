use std::{fs::File, io::BufReader, io::BufRead};

use crate::day::{Day, Answer};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SEEDS_RE: Regex = Regex::new("seeds: ([\\d ]+)").unwrap();
    static ref SEED_RE: Regex = Regex::new("([\\d]+)").unwrap();
    static ref MAP_HEADING_RE: Regex = Regex::new("([a-z\\-]+) map:").unwrap();
    static ref CO_RANGE_RE: Regex = Regex::new("([\\d]+) ([\\d]+) ([\\d]+)").unwrap();
}

#[derive(Debug)]
struct CoRange {
    start1: usize,
    start2: usize,
    len: usize,
}

struct Input {
    seeds: Vec<usize>,
    mappings: [Vec<CoRange>; 7],
}

pub struct Day5<'a> {
    input_filename: &'a str,
}

impl<'a> Day5<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }

    fn read_input(&self, _part2: bool) -> Input {

        let mut seeds: Vec<usize> = Vec::new();
        let mut mappings: [Vec<CoRange>; 7] = [
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
            Vec::new(), Vec::new(), Vec::new()
        ];

        let mut curr_mapping: usize = 0;

        let infile = File::open(&self.input_filename).expect("Failed to open puzzle input.");

        let reader = BufReader::new(infile);

        for line in reader.lines() {
            match line {
                Err(_) => break,
                Ok(line) => {
                    if let Some(caps) = SEEDS_RE.captures(&line) {
                        // parse and store seeds
                        seeds = SEED_RE.captures_iter(&caps[1]).map(|m| m[1].parse().unwrap()).collect();
                    }
                    else if let Some(caps) = MAP_HEADING_RE.captures(&line) {
                        // Set index of the mapping we're collecting.
                        curr_mapping = match &caps[1] {
                            "seed-to-soil" => 0,
                            "soil-to-fertilizer" => 1,
                            "fertilizer-to-water" => 2,
                            "water-to-light" => 3,
                            "light-to-temperature" => 4,
                            "temperature-to-humidity" => 5,
                            "humidity-to-location" => 6,
                            _ => panic!("Invalid mapping name."),
                        }
                    }
                    else if let Some(caps) = CO_RANGE_RE.captures(&line) {
                        // Store a new CoRange
                        let start1 = caps[1].parse().unwrap();
                        let start2 = caps[2].parse().unwrap();
                        let len = caps[3].parse().unwrap();

                        mappings[curr_mapping].push(CoRange { start1, start2, len});
                    }
                }
            }
        }
        
        Input { seeds, mappings }
    }

    // returns (location, range_len) 
    // location - the location mapped to by input seed.
    // range_len - the distance to the next discontinuity.
    fn seed_location(&self, input: &Input, seed: usize) -> (usize, usize) {
        let mut n = seed;
        let mut range = std::usize::MAX;

        for mapping in 0..7 {
            // Look through the rules, to map this to a new range, otherwise
            // destination is the same.
            for co_range in &input.mappings[mapping] {
                if n >= co_range.start2 && n < co_range.start2+co_range.len {

                    let new_range = co_range.len - (n - co_range.start2);
                    n = n - co_range.start2 + co_range.start1;
                    
                    if new_range < range {
                        range = new_range;
                    }
                    break;
                }
            }
        }

        (n, range)
    }

    fn min_location(&self, input: &Input, start: usize, len: usize) -> usize {
        let mut min = std::usize::MAX;

        let mut current = start;
        while current < start+len {
            // Get location *and range_len* of seed <current>
            // The range is the number of locations up from current
            // to the next discontinuity
            let (location, range_len) = self.seed_location(input, current);

            if location < min {
                min = location;
            }
            
            current += range_len;
        }

        min
    }


}

impl<'a> Day for Day5<'a> {
    fn part1(&self) -> Answer {
        let input = self.read_input(false);

        let min = input.seeds.iter().map(|seed| self.seed_location(&input, *seed)).min().unwrap();
        Answer::Numeric(min.0)
    }

    fn part2(&self) -> Answer {
        let input = self.read_input(false);

        assert!(input.seeds.len() % 2 == 0);

        let mut min = std::usize::MAX;

        for n in (0..input.seeds.len()).step_by(2) {
            let start = input.seeds[n];
            let len = input.seeds[n+1];

            let range_min = self.min_location(&input, start, len);
            if range_min < min {
                min = range_min;
            }
        }

        Answer::Numeric(min)
    }
}

#[cfg(test)]
mod tests {

    use crate::{Day, Answer, Day5};

    #[test]
    fn test_input_p1() {        
        let d = Day5::new("examples/day5_example1.txt");
        let input = d.read_input(false);

        assert_eq!(input.seeds.len(), 4);
        println!("Mappings 0? {:?}", input.mappings[0]);
        assert_eq!(input.mappings[0].len(), 2);
        assert_eq!(input.mappings[6][1].start1, 56);
        assert_eq!(input.mappings[6][1].start2, 93);
        assert_eq!(input.mappings[6][1].len, 4);
    }

    #[test]
    fn test_seed_location() {
        let d = Day5::new("examples/day5_example1.txt");
        let input = d.read_input(false);

        assert_eq!(d.seed_location(&input, 79), (82, 3));
        assert_eq!(d.seed_location(&input, 14), (43, 1));
        assert_eq!(d.seed_location(&input, 55), (86, 4));
        assert_eq!(d.seed_location(&input, 13), (35, 1));
    }

    #[test]
    fn test_min_location() {
        let d = Day5::new("examples/day5_example1.txt");
        let input = d.read_input(false);

        
        assert_eq!(d.min_location(&input, 79, 14), 46);
        assert_eq!(d.min_location(&input, 55, 13), 56);
    }

    #[test]
    fn test_p1() {
        let d = Day5::new("examples/day5_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(35));
    }

    #[test]
    fn test_p2() {
        let d = Day5::new("examples/day5_example1.txt");

        assert_eq!(d.part2(), Answer::Numeric(46));
    }
}
