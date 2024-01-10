use std::{io::{BufReader, BufRead}, fs::File, fmt::{Formatter, Error}, fmt::Debug, collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use crate::day::{Day, Answer};

#[derive(PartialEq, Eq, Debug, Hash)]
enum Occupation {
    Empty,
    FixedRock,
    MovableRock,
}

#[derive(Hash)]
struct Table {
    cells: Vec<Vec<Occupation>>,
}

impl Debug for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in 0..self.cells.len() {
            for col in 0..self.cells[0].len() {
                match self.cells[row][col] {
                    Occupation::Empty => {
                        write!(f, ".")?;
                    }
                    Occupation::FixedRock => {
                        write!(f, "#")?;
                    }
                    Occupation::MovableRock => {
                        write!(f, "O")?;
                    }
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}


impl Table {
    fn tilt_north(&mut self) {
        for start_row in 0..self.cells.len() {
            for col in 0..self.cells[0].len() {
                if self.cells[start_row][col] == Occupation::MovableRock {
                    let mut dest_row = start_row;
                    while dest_row > 0 && self.cells[dest_row-1][col] == Occupation::Empty {
                        // The rock will move
                        dest_row -= 1;
                    }
                    self.cells[start_row][col] = Occupation::Empty;
                    self.cells[dest_row][col] = Occupation::MovableRock;
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for start_col in 0..self.cells[0].len() {
            for row in 0..self.cells.len() {
                if self.cells[row][start_col] == Occupation::MovableRock {
                    let mut dest_col = start_col;
                    while dest_col > 0 && self.cells[row][dest_col-1] == Occupation::Empty {
                        // The rock will move
                        dest_col -= 1;
                    }
                    self.cells[row][start_col] = Occupation::Empty;
                    self.cells[row][dest_col] = Occupation::MovableRock;
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for start_row in (0..self.cells.len()).rev() {
            for col in 0..self.cells[0].len() {
                if self.cells[start_row][col] == Occupation::MovableRock {
                    let mut dest_row = start_row;
                    while dest_row < self.cells.len()-1 && self.cells[dest_row+1][col] == Occupation::Empty {
                        // The rock will move
                        dest_row += 1;
                    }
                    self.cells[start_row][col] = Occupation::Empty;
                    self.cells[dest_row][col] = Occupation::MovableRock;
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for start_col in (0..self.cells[0].len()).rev() {
            for row in 0..self.cells.len() {
                if self.cells[row][start_col] == Occupation::MovableRock {
                    let mut dest_col = start_col;
                    while dest_col < self.cells[0].len()-1 && self.cells[row][dest_col+1] == Occupation::Empty {
                        // The rock will move
                        dest_col += 1;
                    }
                    self.cells[row][start_col] = Occupation::Empty;
                    self.cells[row][dest_col] = Occupation::MovableRock;
                }
            }
        }
    }

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn spin_multiple(&mut self, limit: usize) {
        let mut hashes: HashMap<u64, usize> = HashMap::new();

        let mut hasher = DefaultHasher::new();
        let mut count = 0;

        // generate hash, store this value with count 0.
        self.hash(&mut hasher);
        let hash_value = hasher.finish();
        hashes.insert(hash_value, count);

        // search for the repeating period in the following loop, until it's found.
        let mut searching = true;

        while count < limit {
            self.spin_cycle();
            count += 1;

            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            let hash_value = hasher.finish();



            if searching && hashes.contains_key(&hash_value) {
                // We found a repeating cycle.  Fast forward N cycles, where N
                // is a multiple of the period and count+N*period <= limit
                let period = count - hashes.get(&hash_value).unwrap();

                let ff = ((limit - count) / period) * period;
                // println!("At count:{count}, found period:{period}, fast forward: {ff}");

                count += ff;

                // No longer searching for the period
                searching = false;
            }

            hashes.insert(hash_value, count);

            // println!("{:?}", self);
            // println!("Count: {count} Hash value: {hash_value}, Load: {}", self.north_load());
            // println!();
        }
    }

    fn north_load(&self) -> usize {
        let mut total_load = 0;

        for row in 0..self.cells.len() {
            let single_load = self.cells.len() - row;
            for col in 0..self.cells[0].len() {
                total_load += match self.cells[row][col] {
                    Occupation::MovableRock => {
                        single_load
                    }
                    Occupation::FixedRock => {
                        0
                    }
                    Occupation::Empty => {
                        0
                    }
                }
            }
        }

        total_load
    }
}

struct Input {
    table: Table
}

impl Input {
    pub fn read(filename: &str) -> Input 
    {
        let mut cells: Vec<Vec<Occupation>> = Vec::new();

        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let mut row: Vec<Occupation> = Vec::new();
            if let Ok(line) = line {
                for c in line.chars() {
                    row.push(
                        match c {
                            '.' => Occupation::Empty,
                            '#' => Occupation::FixedRock,
                            'O' => Occupation::MovableRock,
                            _ => panic!("Unexpected character in input."),
                        });
                }
            }
            cells.push(row);
        }

        let table = Table {cells};
        Input { table }
    }
}

pub struct Day14<'a> {
    input_filename: &'a str,
}

impl<'a> Day14<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }
}

impl<'a> Day for Day14<'a> {
    fn part1(&self) -> Answer {
        let mut input = Input::read(self.input_filename);
        input.table.tilt_north();

        let total_load = input.table.north_load();

        Answer::Numeric(total_load)
    }

    fn part2(&self) -> Answer {
        let mut input = Input::read(self.input_filename);
        input.table.spin_multiple(1000000000);

        let total_load = input.table.north_load();

        Answer::Numeric(total_load)
    }
}

#[cfg(test)]
mod test {
    use crate::day14::{Occupation, Input};

    #[test]
    fn test_input() {
        let input = Input::read("examples/day14_example1.txt");
        assert_eq!(input.table.cells.len(), 10);
        assert_eq!(input.table.cells[0].len(), 10);
        assert_eq!(input.table.cells[0][0], Occupation::MovableRock);
        assert_eq!(input.table.cells[0][5], Occupation::FixedRock);
        assert_eq!(input.table.cells[2][0], Occupation::Empty);
    }

    #[test]
    fn test_tilt_load() {
        let mut input = Input::read("examples/day14_example1.txt");
        input.table.tilt_north();
        // println!("Tilted: ");
        // println!("{:?}", input.table);

        let total_load = input.table.north_load();

        assert_eq!(total_load, 136);
    }


    #[test]
    fn test_spin() {
        let mut input = Input::read("examples/day14_example1.txt");
        for _ in 0..3 {
            input.table.spin_cycle();
            // println!("Spin: ");
            // println!("{:?}", input.table);
            // println!("Load: {}", input.table.north_load());
            // println!();
        }

        let total_load = input.table.north_load();

        assert_eq!(total_load, 69);
    }


    #[test]
    fn test_spin_multiple() {
        let mut input = Input::read("examples/day14_example1.txt");
  
        input.table.spin_multiple(1000000000);
        // println!("Spin: ");
        // println!("{:?}", input.table);
        // println!("Load: {}", input.table.north_load());
        // println!();

        let total_load = input.table.north_load();

        assert_eq!(total_load, 64);
    }
}