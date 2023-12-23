use std::{fs::File, io::{BufReader, BufRead}};

use crate::day::{Day, Answer};

struct Input {
    cols: usize,
    rows: usize,
    galaxy_positions: Vec<(usize, usize)>,
}

pub struct Day11<'a> {
    _input_filename: &'a str,
}

impl<'a> Day11<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }

    fn read_input(filename: &str) -> Input {
        let mut rows = 0;
        let mut cols = 0;
        let mut galaxy_positions: Vec<(usize, usize)> = Vec::new();

        let infile = File::open(filename).expect("Failed to open puzzle input.");

        let reader = BufReader::new(infile);
        let mut row_no = 0;
        for line in reader.lines() {
            let mut col_no = 0;
            for c in line.unwrap().chars() {
                if c == '#' {
                    galaxy_positions.push((row_no, col_no));
                }
                col_no += 1;
            }

            row_no += 1;
            rows = row_no;
            cols = col_no;
        }

        Input { cols, rows, galaxy_positions }
    }

    fn dist_sums(input: &Input, expansion: isize) -> usize {
        // create vectors of col_widths and row_widths, initialize all to two.
        let mut col_width: Vec<isize> = vec![expansion; input.cols];
        let mut row_width: Vec<isize> = vec![expansion; input.rows];
        
        // rows and columns that are non-empty should have width 1
        for (row, col) in &input.galaxy_positions {
            col_width[*col] = 1;
            row_width[*row] = 1;
        }

        // create vectors of col_pos, row_pos.
        let mut col_pos: Vec<isize> = vec![0; input.cols];
        let mut row_pos: Vec<isize> = vec![0; input.rows];
        let mut pos = 0;
        for col_no in 0.. input.cols {
            col_pos[col_no] = pos;
            pos += col_width[col_no];
        }

        let mut pos = 0;
        for row_no in 0.. input.rows {
            row_pos[row_no] = pos;
            pos += row_width[row_no];
        }

        // For all pairs of galaxies...
        let mut total_dist: isize = 0;
        for galaxy1 in 0..input.galaxy_positions.len()-1 {
            for galaxy2 in galaxy1+1 .. input.galaxy_positions.len() {
                let (row1, col1) = input.galaxy_positions[galaxy1];
                let (row2, col2) = input.galaxy_positions[galaxy2];
                total_dist += (row_pos[row1] - row_pos[row2]).abs() + 
                              (col_pos[col1] - col_pos[col2]).abs();
            }
        }
        
        total_dist as usize
    }
}

impl<'a> Day for Day11<'a> {
    fn part1(&self) -> Answer {
        let input = Day11::read_input(self._input_filename);

        Answer::Numeric(Day11::dist_sums(&input, 2))
    }

    fn part2(&self) -> Answer {
        let input = Day11::read_input(self._input_filename);

        Answer::Numeric(Day11::dist_sums(&input, 1000000))
    }
}

#[cfg(test)]
mod test {
    use crate::{day11::Day11, day::{Day, Answer}};

    #[test]
    fn test_input() {
        let input = Day11::read_input("examples/day11_example1.txt");

        assert_eq!(input.rows, 10);
        assert_eq!(input.cols, 10);
        assert_eq!(input.galaxy_positions.len(), 9);
        assert_eq!(input.galaxy_positions[0], (0, 3));
        assert_eq!(input.galaxy_positions[8], (9, 4));
    }

    
    #[test]
    fn test_distance_sum() {
        let input = Day11::read_input("examples/day11_example1.txt");

        assert_eq!(Day11::dist_sums(&input, 2), 374);
    }

    #[test]
    fn test_distance_sum_p2() {
        let input = Day11::read_input("examples/day11_example1.txt");

        assert_eq!(Day11::dist_sums(&input, 100), 8410);
    }

    #[test]
    fn test_part1() {
        let d = Day11::new("examples/day11_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(374));
    }
}
