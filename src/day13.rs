use std::{fs::File, io::{BufReader, BufRead}};

use crate::day::{Day, Answer};

struct Pattern {
    width: usize,
    height: usize,
    rows: Vec<Vec<char>>,
}

impl Pattern {
    fn new(rows: &Vec<String>) -> Pattern {
        let height = rows.len();
        let width = rows[0].len();

        // TODO: convert rows to Vec<Vec<char>>
        let mut new_rows: Vec<Vec<char>> = Vec::new();
        for row_str in rows {
            let mut new_row: Vec<char> = Vec::new();
            for c in row_str.chars() {
                new_row.push(c);
            }
            new_rows.push(new_row);
        }

        Pattern { width, height, rows: new_rows }
    }

    fn test_vert_reflect(&self, pos: usize, smudges: usize) -> bool {
        // println!("Test vert reflect on w{} x h{} at pos:{}", self.width, self.height, pos);

        if pos < 1 || pos >= self.width {
            return false;
        }

        let mut mismatches = 0;

        let cols_to_check = if pos <= self.width/2 {
            pos
        }
        else {
            self.width-pos
        };

        for n in 0..cols_to_check {
            // println!("pos: {pos}, n: {n}");

            let right_col = pos+n;
            let left_col = pos-n-1;
            for y in 0..self.height {
                if self.rows[y][left_col] != self.rows[y][right_col] {
                    // found a mismatch
                    mismatches += 1;
                }
            }
        }

        // This is a reflection if the number of mismatches equals the number of smudges.
        mismatches == smudges
    }

    fn test_hor_reflect(&self, pos: usize, smudges: usize) -> bool {
        if pos < 1 || pos >= self.height {
            return false;
        }

        let mut mismatches = 0;

        let rows_to_check = if pos <= self.height/2 {
            pos
        }
        else {
            self.height-pos
        };

        for n in 0..rows_to_check {
            let bott_row = pos+n;
            let top_row = pos-1-n;
            for x in 0..self.width {
                if self.rows[bott_row][x] != self.rows[top_row][x] {
                    // found a mismatch
                    mismatches += 1;
                }
            }
        }

        // A reflection is found if mismatches == expected smudges
        mismatches == smudges
    }

    fn reflection(&self, smudges: usize) -> usize {
        // Look for vertical reflection
        for pos in 1..=self.width-1 {
            if self.test_vert_reflect(pos, smudges) {
                return pos;
            }
        }

        // Look for horizontal reflection
        for pos in 1..=self.height-1 {
            if self.test_hor_reflect(pos, smudges) {
                return 100*pos;
            }
        }

        // No reflections found.
        panic!();
    }
}

struct Input {
    patterns: Vec<Pattern>,
}

impl Input {
    fn read(filename: &str) -> Input {
        let infile = File::open(filename).expect("Failed to open puzzle input.");
        let mut patterns: Vec<Pattern> = Vec::new();
        let mut rows: Vec<String> = Vec::new();

        let reader = BufReader::new(infile);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.len() > 0 {
                    // Add a row to rows
                    rows.push(line.to_string());
                }
                else {
                    // blank line. At the end of the group, create a Pattern
                    let pattern = Pattern::new(&rows);
                    patterns.push(pattern);

                    // Reset for the next group.
                    rows.clear();
                }
            }
        }

        if rows.len() > 0 {
            // Create the last group
            let pattern = Pattern::new(&rows);
            patterns.push(pattern);
            rows.clear();
        }

        Input { patterns }
    }

    fn sum_reflections(&self, smudges: usize) -> usize {
        self.patterns.iter().map(|p| p.reflection(smudges)).sum()
    }
}

pub struct Day13<'a> {
    input_filename: &'a str,
}

impl<'a> Day13<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }
}

impl<'a> Day for Day13<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self.input_filename);

        Answer::Numeric(input.sum_reflections(0))
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self.input_filename);

        Answer::Numeric(input.sum_reflections(1))
    }
}

#[cfg(test)]
mod test {
    use crate::{day13::Day13, day::{Day, Answer}};

    use super::Input;

    #[test]
    fn test_input() {
        let input = Input::read("examples/day13_example1.txt");
        assert_eq!(input.patterns.len(), 2);
        assert_eq!(input.patterns[0].width, 9);
        assert_eq!(input.patterns[0].height, 7);
        assert_eq!(input.patterns[1].width, 9);
        assert_eq!(input.patterns[1].height, 7);
    }

    #[test]
    fn test_reflection() {
        let input = Input::read("examples/day13_example1.txt");
        assert_eq!(input.patterns[0].test_vert_reflect(5, 0), true);
        assert_eq!(input.patterns[0].test_vert_reflect(4, 0), false);
        assert_eq!(input.patterns[0].test_vert_reflect(6, 0), false);
        assert_eq!(input.patterns[0].reflection(0), 5);       

        assert_eq!(input.patterns[1].test_hor_reflect(4, 0), true);
        assert_eq!(input.patterns[1].test_hor_reflect(5, 0), false);
        assert_eq!(input.patterns[1].test_hor_reflect(3, 0), false);
        assert_eq!(input.patterns[1].reflection(0), 400);  
    }

    #[test]
    fn test_reflection2() {
        let input = Input::read("examples/day13_example1.txt");
        assert_eq!(input.patterns[0].test_hor_reflect(3, 1), true);
        assert_eq!(input.patterns[0].test_hor_reflect(2, 1), false);
        assert_eq!(input.patterns[0].test_hor_reflect(4, 1), false);
        assert_eq!(input.patterns[0].reflection(1), 300);       

        assert_eq!(input.patterns[1].test_hor_reflect(1, 1), true);
        assert_eq!(input.patterns[1].test_hor_reflect(2, 1), false);
        assert_eq!(input.patterns[1].reflection(1), 100);  
    }

    #[test]
    fn test_sum_reflection() {
        let input = Input::read("examples/day13_example1.txt");
        assert_eq!(input.sum_reflections(0), 405);
    }

    #[test]
    fn test_sum_reflection2() {
        let input = Input::read("examples/day13_example1.txt");
        assert_eq!(input.sum_reflections(1), 400);
    }
        
    #[test]
    fn test_part1() {
        let d = Day13::new("examples/day13_example1.txt");
        
        assert_eq!(d.part1(), Answer::Numeric(405));
    }

    #[test]
    fn test_part2() {
        let d = Day13::new("examples/day13_example1.txt");
        
        assert_eq!(d.part2(), Answer::Numeric(400));
    }
}
