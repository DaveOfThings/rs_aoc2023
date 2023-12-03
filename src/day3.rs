use crate::day::{Day, Answer, LineBasedInput};
use std::fs::File;

struct Line {
    text: String,
}


struct Number {
    number: usize,
    col: usize, 
    row: usize,
    len: usize,
}

struct Sym {
    c: char,
    col: usize, 
    row: usize  // TODO-DW
}

struct Input {
    symbols: Vec<Sym>,
    numbers: Vec<Number>,
}

impl Input {
    // determine if a particular number is adjacent to a particular symbol
    fn adjacent(&self, number: &Number, sym: &Sym) -> bool {
        let mut is_adjacent = false;
        let target_c1 = number.col;
        let target_c2 = number.col+number.len-1;

        if (sym.row == number.row) || (sym.row+1 == number.row) || (sym.row == number.row+1) {
            // row is in range
            if (sym.col+1 >= target_c1) && (sym.col <= target_c2+1) {
                // col is in range
                is_adjacent = true;
            }
        }

        is_adjacent
    }

    // A number is a part number if adjacent to some symbol.
    fn is_part_number(&self, candidate: &Number)-> bool {
        let mut is = false;
        for sym in &self.symbols {
            if self.adjacent(candidate, sym) {
                is = true;
                break;
            }
        }

        is
    }

    // Check if a symbol is a gear (a '*' adjacent to exactly two part numbers)
    // If so, return Some(gear ratio) else None
    fn gear_ratio(&self, sym: &Sym) -> Option<usize> {
        if sym.c == '*' {
            // Get a vector of all numbers adjacent to this symbol
            let adj_numbers: Vec<&Number> = self.numbers.iter()
                .filter(|n| self.adjacent(n, sym))
                .collect();

            if adj_numbers.len() == 2 {
                // We have exactly two adjacent numbers
                Some(adj_numbers[0].number * adj_numbers[1].number)
            }
            else {
                // Wrong number of adjacent numbers
                None
            }
        }
        else {
            // Symbol isn't '*', this isn't a gear.
            None
        }
    }

    fn sum_part_numbers(&self) -> usize {
        self.numbers.iter()
            .filter(|n| self.is_part_number(n))
            .map(|n| n.number)
            .sum()
    }

    fn sum_gear_ratios(&self) -> usize {
        self.symbols.iter()              // iterate over all symbols
            .map(|s| self.gear_ratio(s))    // get each gear ratio as Option<usize>
            .filter_map(|x| x)    // drop invalid ones
            .sum()
    }
}


pub struct Day3 {
    input_filename: String,
}

impl Day3 {
    pub fn new(filename: &str) -> Self {
        Self { input_filename: filename.to_string() }
    }

    fn read_input(&self, _part2: bool) -> Input {
        let infile = File::open(&self.input_filename).expect("Failed to open puzzle input.");
        let lines = self.process(infile, false);

        // Locate symbols
        let mut symbols: Vec<Sym> = Vec::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.text.chars().enumerate() {
                if "!@#$%^&*()_+-=/?><|\\}{][`~\"':;".contains(c) {
                    symbols.push( Sym {c, col, row} );
                }
            }
        }

        // Locate numbers
        let mut numbers: Vec<Number> = Vec::new();
        for (row, line) in lines.iter().enumerate() {
            let mut in_num = false;
            let mut value = 0;
            let mut len = 0;
            let mut start_col = 0;

            for (col, c) in line.text.chars().enumerate() {
                if !in_num {
                    if "0123456789".contains(c) {
                        // This is the start of a string of digits
                        start_col = col;
                        value = c.to_digit(10).unwrap() as usize;
                        len = 1;
                        in_num = true;
                    }
                    else {
                        // scan past non-digits
                        in_num = false;
                    }
                }
                else {
                    if "0123456789".contains(c) {
                        // Add a digit to the current value
                        value = value * 10 + c.to_digit(10).unwrap() as usize;
                        len += 1;
                        in_num = true;
                    }
                    else {
                        // Reached the end of a string of digits
                        numbers.push(Number {number: value, col: start_col, row: row, len: len });
                        in_num = false;
                    }
                }
            }

            // Reached end of column, if a number was in progress, store it
            if in_num {
                numbers.push(Number {number: value, col: start_col, row: row, len: len });
            }
        }
        
        Input { symbols, numbers }
    }




}

impl LineBasedInput<Line> for Day3 {
    fn parse_line(line: &str, _part2: bool) -> Option<Line> {
        Some(Line {text: line.to_string()})
    }
}

impl Day for Day3 {


    fn part1(&self) -> Answer {
        let input = self.read_input(false);

        Answer::Numeric(input.sum_part_numbers())
    }

    fn part2(&self) -> Answer {
        let input = self.read_input(true);

        Answer::Numeric(input.sum_gear_ratios())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day, Answer, Day3};
    // use crate::day3::Record;

    #[test]
    fn test_input_p1() {        
        let d = Day3::new("examples/day3_example1.txt");
        let input = d.read_input(false);

        assert_eq!(input.symbols.len(), 6);
        assert_eq!(input.numbers.len(), 10);
    }

    #[test]
    fn test_input_p2() {        
        let d = Day3::new("examples/day3_example1.txt");
        let input = d.read_input(true);

        assert_eq!(input.symbols.len(), 6);
        assert_eq!(input.numbers.len(), 10);
    }

    #[test]
    fn test_is_part_number() {        
        let d = Day3::new("examples/day3_example1.txt");
        let input = d.read_input(true);

        // 8 of the numbers in the example are part numbers.  (All but two.)
        assert_eq!(input.numbers.iter().filter(|n| input.is_part_number(n)).count(), 8);
    }

    #[test]
    fn test_part1() {
        let d = Day3::new("examples/day3_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(4361));
    }

    #[test]
    fn test_is_gear() {
        let d = Day3::new("examples/day3_example1.txt");
        let input = d.read_input(true);

        let gear_ratios: Vec<usize> = input.symbols.iter()              // iterate over all symbols
            .map(|s| input.gear_ratio(s))    // get each gear ratio as Option<usize>
            .filter_map(|x| x).collect();

        assert_eq!(gear_ratios.len(), 2);  
    }

    #[test]
    fn test_part2() {
        let d = Day3::new("examples/day3_example1.txt");
        assert_eq!(d.part2(), Answer::Numeric(467835));
    }
}
