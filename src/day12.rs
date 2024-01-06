use std::{io::{BufReader, BufRead}, fs::File};

use crate::day::{Day, Answer};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RECORD_RE: Regex = Regex::new("([\\.\\#\\?]+) ([0-9,]+)").unwrap();
    static ref START_Q_RE: Regex = Regex::new("^(\\.*[\\?])").unwrap();
    static ref EMPTY_RE: Regex = Regex::new("^([\\.]*)$").unwrap();

    // Each of these RE's matches if the given string can match a run of N characters.
    static ref RUN0_RE: Regex = Regex::new("^([\\.\\?]*)$").unwrap();
    static ref RUN1_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{1}[\\.\\?]*)$").unwrap();
    static ref RUN2_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{2}[\\.\\?]*)$").unwrap();
    static ref RUN3_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{3}[\\.\\?]*)$").unwrap();
    static ref RUN4_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{4}[\\.\\?]*)$").unwrap();
    static ref RUN5_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{5}[\\.\\?]*)$").unwrap();
    static ref RUN6_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{6}[\\.\\?]*)$").unwrap();
    static ref RUN7_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{7}[\\.\\?]*)$").unwrap();
    static ref RUN8_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{8}[\\.\\?]*)$").unwrap();
    static ref RUN9_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{9}[\\.\\?]*)$").unwrap();
    static ref RUN10_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{10}[\\.\\?]*)$").unwrap();
    static ref RUN11_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{11}[\\.\\?]*)$").unwrap();
    static ref RUN12_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{12}[\\.\\?]*)$").unwrap();
    static ref RUN13_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{13}[\\.\\?]*)$").unwrap();
    static ref RUN14_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{14}[\\.\\?]*)$").unwrap();
    static ref RUN15_RE: Regex = Regex::new("^([\\.\\?]*[\\#\\?]{15}[\\.\\?]*)$").unwrap();
    static ref RUN_RES: Vec<&'static Regex> = vec![
        &RUN0_RE,
        &RUN1_RE,
        &RUN2_RE,
        &RUN3_RE,
        &RUN4_RE,
        &RUN5_RE,
        &RUN6_RE,
        &RUN7_RE,
        &RUN8_RE,
        &RUN9_RE,
        &RUN10_RE,
        &RUN11_RE,
        &RUN12_RE,
        &RUN13_RE,
        &RUN14_RE,
        &RUN15_RE,
    ];

}

/*
Thoughts on counting matches.  Use run lengths when resolving ways the string can form.
When not in a run and a '?' is encountered, in order for it to be a '#', the next N characters
have to fit the run length.  That is they must be '#' or '?' and the character after that must
be a '.' or '?'.  If we go down that path, all of the '?' marks are then fixed to the necessary
values.

In order for a '?' to be a '.' when not in a run, ... well, treat it like it could be.

The recursion should end with combinations of zero if a run length can't be satisfied.  That
is if a '.' is encountered too soon after a '#' or the N+1th char is another '#'.

The recursion could be passed a shorter string and shorter list of runs as runs are matched
to their run lengths.

Basically, the recursion will happen one run at a time, not one character at a time.

So, at each level, start by scanning past '.' to a '?' or '#'
set combos to 0
If we're at a possible run, N '?' or '#' chars followed by a '.' or '?':
    treat those as a run and recursively count combos on the shorted criteria.
    combos += recursive call
If the first char was '?', treat it as a '.' and recursively count combos on shortened string
    combos += recursive call

return combos.  (Will be zero if we hit a conflict.)

*/

struct Record {
    condition: String,
    runs: Vec<usize>,
}

impl Record {
    fn new(s: &str) -> Record {
        let captures = RECORD_RE.captures(s).unwrap();

        let condition = captures[1].to_string();
        let runs = 
            captures[2].split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Record { condition, runs }
    }


    /*
    // Looks at runs up to the first question mark
    // Returns a Vec<usize> of the run lengths.
    // Returns true if the last element is definite, false if it could grow.
    fn prefix_runs(condition: &str) -> (Vec<usize>, bool) {
        let mut runs: Vec<usize> = Vec::new();
        let mut definite = false;
        let mut run_len = 0;
        let mut in_run = false;

        for c in condition.chars() {
            match c {
                '.' => {
                    if in_run {
                        // A run ended
                        runs.push(run_len);
                        run_len = 0;
                        definite = true;
                        in_run = false;
                    }
                }
                '#' => {
                    if in_run {
                        // The run continues
                        run_len += 1;
                    }
                    else {
                        // Start a run
                        run_len = 1;
                        definite = true;
                        in_run = true;
                    }
                }
                '?' => {
                    // stop processing
                    if in_run {
                        runs.push(run_len);
                        definite = false;
                        in_run = false;
                    }
                    break;
                }
                _ => {
                    // Bad characters in condition string
                    panic!();
                }
            }
        }

        if in_run {
            runs.push(run_len);
        }

        (runs, definite)
    }


    fn sub_arrangements(&self, condition: &str) -> usize {
        self._sub_arrangements(condition, 0)
    }

    fn _sub_arrangements(&self, condition: &str, level: usize) -> usize {
        println!("{level}: sub_arrangements of {}, {:?}", condition, self.runs);

        // Check compatibility of this condition's prefix with the puzzle condition.
        let (prefix_runs, definite) = Record::prefix_runs(condition);

        // All but the last element of prefix_runs need to match exactly
        let len = prefix_runs.len();

        if len > self.runs.len() {
            // too many runs produced
            println!("    Pruned: too many runs.");
            return 0;
        }

        if len > 0 {
            for n in 0..len-1 {
                if prefix_runs[n] != self.runs[n] { 
                    println!("    Pruned: mismatch in first N-1");
                    return 0; 
                }
            }

            // The last element needs to match exactly if definite, otherwise <= puzzle condition
            if definite {
                if prefix_runs[len-1] != self.runs[len-1] { 
                    println!("    Pruned: definite last elt is a mismatch");
                    return 0; 
                }
            }
            else {
                if prefix_runs[len-1] > self.runs[len-1] {
                    println!("    Pruned: indefinite last elt is a mismatch.");
                    return 0; 
                }
            }
        }

        let mut s1 = String::new();
        let mut s2 = String::new();
        let mut subbed = false;
        for c in condition.chars() {
            if c == '?' && !subbed {
                // Substitute for the '?'
                s1.push('.');
                s2.push('#');
                subbed = true;
            }
            else {
                s1.push(c);
                s2.push(c);
            }
        }

        if !subbed {
            // No more ambiguity
            let found_runs = Record::runs(condition);
            if found_runs == self.runs {
                // This is a match!
                1
            }
            else {
                // Not a match
                0
            }
        }
        else {
            let arrangements1 = self._sub_arrangements(&s1, level+1);
            let arrangements2 = self._sub_arrangements(&s2, level+1);
            // println!("  s1: {s1} -> {arrangements1}");
            // println!("  s2: {s2} -> {arrangements2}");
            // println!("  Adding {arrangements1} + {arrangements2}");
            arrangements1 + arrangements2
        }
    }

    */

    fn runs(s: &str) -> Vec<usize> {
        let mut retval: Vec<usize> = Vec::new();

        let mut in_run = false;
        let mut run_len = 0;

        for c in s.chars() {
            if c == '#' {
                if in_run {
                    run_len += 1;
                }
                else {
                    in_run = true;
                    run_len = 1;
                }
            }
            else {
                if in_run {
                    retval.push(run_len);
                    in_run = false;
                }
            }
        }
        if in_run {
            retval.push(run_len);
        }

        retval
    }

    fn arrangements(&self) -> usize {
        let mut arrangements = 0;

        let n = self.condition.chars()
            .filter(|c| c == &'?')
            .count();

        // println!("Found {} unknowns.", n);

        let mut candidate = String::new();
        for seed in 0..(1<<n) {

            let mut shifted = seed;
            candidate.clear();
            for c in self.condition.chars() {
                if c == '?' {
                    if shifted & 1 == 1 {
                        candidate.push('#');
                    }
                    else {
                        candidate.push('.');
                    }
                    shifted >>= 1;
                }
                else {
                    candidate.push(c);
                }
            }

            let candidate_runs = Record::runs(&candidate);
            if candidate_runs == self.runs {
                arrangements += 1;
            }
        }

        arrangements

    }

    // This works recursively with sub-calls passing shorter condition strings and
    // run vectors.  We can't pass in a condition string in the middle of a run.  It
    // is assumed that the first '#' of condition is the start of a run.
    fn _arrangements2(level: usize, condition: &str, runs: &[usize]) -> usize {
        // let spaces = std::iter::repeat(" ").take(level*2).collect::<String>();
        
        let cond_vec: Vec<char> = Vec::from(condition.chars().collect::<Vec<char>>());
        let mut arrangements = 0;

        // Handle end cases: empty condition, empty runs
        if runs.len() == 0 {
            if RUN0_RE.is_match(condition) { 
                // The condition supports no runs and that's what we want.
                // println!("{} Found a working arrangement.", spaces);
                return 1; 
            }
            else {
                // The condition requires another run but we can't have it.
                // println!("{} Condition has runs, vector has none.", spaces);
                return 0;
            }
        }

        /*
        if runs.len() == 1 {
            let run_re = RUN_RES[runs[0]];
            if run_re.is_match(condition) {
                // We have a match
                println!("  Single run condition met!");
                return 1;
            }
            else {
                println!("  Mismatch for single run.");
                return 0;
            }
        }
        */

        // From here, we have runs to satisfy and a condition that can satisfy some
        // println!("{}arrangements2: '{condition}', {:?}", spaces, runs);

        let mid_run = runs.len()/2;
        let run_len = runs[mid_run];
        let cond_len = cond_vec.len();
        let min_pre_len = runs[0..mid_run].iter().fold(0, |n, m| if n > 0 {n + m + 1} else {*m});
        let min_post_len = runs[mid_run+1..].iter().fold(0, |n, m| if n > 0 {n + m + 1} else {*m});

        let mut match_pattern: Vec<char> = Vec::new();
        if min_pre_len > 0 {
            match_pattern.push('.');
        }
        for _ in 0..run_len {
            match_pattern.push('#');
        }
        if min_post_len > 0 {
            match_pattern.push('.');
        }
        let match_len = match_pattern.len();

        let start = min_pre_len;
        let end = cond_len - min_post_len - match_len;
        // println!("{} Checking position from {start} to {cond_len}-{min_post_len}-{match_len}={end}", spaces);

        'position: for position in start..=end {
            for n in 0..match_len {
                // If the pattern doesn't match at this position, move on
                if match_pattern[n] == '.' && cond_vec[position+n] == '#' {
                    continue 'position;
                }
                if match_pattern[n] == '#' && cond_vec[position+n] == '.' {
                    continue 'position;
                }
            }

            let pre_cond = &condition[0..position];
            let pre_runs = &runs[0..mid_run];
            let post_cond = if position+match_len < cond_len {
                &condition[position+match_len..]
            }
            else {
                ""
            };
            let post_runs = &runs[mid_run+1..];
        
            let left_arrangements = Record::_arrangements2(
                level+1,
                    pre_cond, 
                    pre_runs);

            let right_arrangements = Record::_arrangements2(
                level+1,
                    post_cond, 
                    post_runs);

            // println!("{} Adding in {} * {}  = {} arrangements", spaces, 
            //     left_arrangements, 
            //     right_arrangements,
            //     left_arrangements * right_arrangements);
            arrangements += left_arrangements * right_arrangements;
        }

        // println!("{} Returning {arrangements}", spaces);
        arrangements
    }

    fn arrangements2(&self) -> usize {
        Record::_arrangements2(0, &self.condition, &self.runs)
    }

    fn unfold(&self) -> Record {
        let mut unfold_cond = String::new();
        let mut unfold_runs: Vec<usize> = Vec::new();

        unfold_cond += self.condition.as_str();
        for _ in 0..4 {
            unfold_cond.push('?');
            unfold_cond += self.condition.as_str();
        }

        for _ in 0..5 {
            unfold_runs.extend(&self.runs);
        }

        Record { condition: unfold_cond, runs: unfold_runs }
    }
}

struct Input {
    records: Vec<Record>,
}

impl Input {
    fn read(filename: &str) -> Input {
        let infile = File::open(filename).expect("Failed to open puzzle input.");
        let mut records: Vec<Record> = Vec::new();

        let reader = BufReader::new(infile);
        for line in reader.lines() {
            let record = Record::new(&line.unwrap());
            records.push(record);
        }

        Input {records}
    }

    fn sum_arrangements(&self) -> usize {
        self.records.iter()
            .map(|r| r.arrangements())
            .sum()
    }

    
    fn sum_unfolded_arrangements(&self) -> usize {
        self.records.iter()
            .map(|r| {
                    let unfolded = r.unfold(); 
                    let num = unfolded.arrangements2();
                    // println!("{num} arrangements2");
                    num
                })
            .sum()
    }
}

pub struct Day12<'a> {
    _input_filename: &'a str,
}

impl<'a> Day12<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }
}

impl<'a> Day for Day12<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self._input_filename);
        Answer::Numeric(input.sum_arrangements())
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self._input_filename);
        Answer::Numeric(input.sum_unfolded_arrangements())
    }
}

#[cfg(test)]
mod test {
    use crate::{day12::{Input, Record, Day12}, day::{Answer, Day}};

    #[test]
    fn test_input() {
        let input = Input::read("examples/day12_example1.txt");

        assert_eq!(input.records.len(), 6);
        assert_eq!(input.records[0].condition, "???.###");
        assert_eq!(input.records[0].runs.len(), 3);
        assert_eq!(input.records[0].runs[2], 3);
    }

    #[test]
    fn test_arrangements() {
        for (s, n) in [("???.### 1,1,3", 1),
                                    (".??..??...?##. 1,1,3", 4),
                                    ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
                                    ("????.#...#... 4,1,1", 1),
                                    ("????.######..#####. 1,6,5", 4),
                                    ("?###???????? 3,2,1", 10)] {
                let record = Record::new(s);
                assert_eq!(record.arrangements(), n);
        }
    }

    #[test]
    fn test_arrangements2() {
        for (s, n) in [
                (". 1", 0),
                ("# 1", 1),
                ("? 1", 1),
            ] {
                let record = Record::new(s);
                let arrangements2 = record.arrangements2();

                assert_eq!(arrangements2, n);
            }
    }

    // TODO: Fix the commented out cases.  The new approach seems to work in most cases and it's efficient.
    #[test]
    fn test_unfolded_arrangements() {
        for (s, n) in [("???.### 1,1,3", 1),
                       (".??..??...?##. 1,1,3", 16384),
                       ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
                       ("????.#...#... 4,1,1", 16),
                       ("????.######..#####. 1,6,5", 2500),
                       ("?###???????? 3,2,1", 506250)
                       ] {
                let record = Record::new(s);
                let unfolded = record.unfold();

                // println!("Testing {}", unfolded.condition);

                let arrangements = unfolded.arrangements2();

                assert_eq!(arrangements, n);
        }
    }

    #[test]
    fn test_unfolded_arrangement() {
        for (s, n) in [
            // ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 16384),
            // ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            // ("????.#...#... 4,1,1", 16),
            // ("????.######..#####. 1,6,5", 2500),
            // ("?###???????? 3,2,1", 506250)
            ] {
                let record = Record::new(s);
                let unfolded = record.unfold();

                println!("Testing {}", unfolded.condition);

                let arrangements = unfolded.arrangements2();

                assert_eq!(arrangements, n);
        }
    }

    #[test]
    fn test_sum_arrangements() {
        let input = Input::read("examples/day12_example1.txt");

        assert_eq!(input.sum_arrangements(), 21);
    }

    
    #[test]
    fn test_sum_unfolded_arrangements() {
        let input = Input::read("examples/day12_example1.txt");

        assert_eq!(input.sum_unfolded_arrangements(), 525152);
    }

    #[test]
    fn test_part1() {
        let d = Day12::new("examples/day12_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(21))
    }

    /*
    #[test]
    fn test_prefix_runs() {
        let vectors = vec![
            ("###...", (vec![3], true)),               // "###..." -> runs: [3,] definite: true
            (".###...", (vec![3], true)),              // "###..." -> runs: [3,] definite: true
            ("#.##..", (vec![1,2], true)),
            (".#.##..", (vec![1,2], true)),
            ("#.##?.", (vec![1,2], false)),
            (".#.##?.", (vec![1,2], false)),
            ("#.##.?.", (vec![1,2], true)),
            (".#.##.?.", (vec![1,2], true)),
            ("#.##?.###", (vec![1,2], false)),
            (".#.##?.###", (vec![1,2], false)),
            ("#.##.?.###", (vec![1,2], true)),
            (".#.##.?.###", (vec![1,2], true)),
            ];

        for (pattern, expected) in vectors {
            println!("Testing with {}", pattern);

            let result = Record::prefix_runs(pattern);
            assert_eq!(result, expected);
        }
    }
    */
}