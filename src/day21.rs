use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::day::{Day, Answer};

struct Input {
    width: usize,
    height: usize,
    start: (isize, isize),
    garden: HashSet<(isize, isize)>,
}

impl Input {
    fn read(filename: &str) -> Input {
        let mut width: usize = 0;
        let mut height: usize = 0;
        let mut start: (isize, isize) = (0, 0);
        let mut garden: HashSet<(isize, isize)> = HashSet::new();

        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);

        for (y, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            width = line.len();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        // garden patch
                        // println!("Inserting at ({x}, {y})");
                        garden.insert((x as isize, y as isize));
                    }
                    '#' => {
                        // rock
                        // (nothing to do)
                    }
                    'S' => {
                        // start position (garden patch)
                        start = (x as isize, y as isize);
                        garden.insert((x as isize, y as isize));
                    }
                    _ => {
                        // Something illegal.
                        panic!();
                    }
                }
            }
            height += 1;
        }

        // We've baked in assumptions that height and width are the same and start is
        // in the center.
        assert_eq!(start.0 as usize, width/2);
        assert_eq!(start.1 as usize, height/2);
        assert_eq!(start.0, start.1);

        Input { width, height, start, garden }

    }
}


// Different classes of blocks, based on where the propagation started.
#[derive(EnumIter, Debug)]
enum FillFrom {
    // Start in the center
    Center,

    // Started in right, center
    // Each new class goes around the compass counter clockwise
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
}

#[derive(EnumIter, Debug)]
enum FillAmount {
    Least,                  // The tile on the leading edge of the propagation front
    NextLeast,              // A tile with part of the propagation front
    FullOdd,                // A full tile that starts on an odd time step.
    FullEven,               // A full tile that starts on an even time step.

    // Note: Since "Odd" and "Even" denote which time step a tile normally starts on,
    // To evaluate how many places are set after <steps> steps, for a tile, seed it with
    // the starting step then run the minimum even number of steps to fill the tile from 
    // that position.  Then run one more step if <steps> was odd XOR It was FullOdd instead
    // of FullEven.
}

pub struct Day21<'a> {
    input_filename: &'a str,
}

impl<'a> Day21<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }

    fn num_of(start: &FillFrom, amount: &FillAmount, steps: usize, input: &Input) -> usize {
        let w = input.width;

        match start {
            FillFrom::Center => {
                match amount {
                    FillAmount::Least => if steps <= w { 1 } else { 0 },
                    FillAmount::NextLeast => 0,
                    FillAmount::FullOdd => 0,
                    FillAmount::FullEven => if steps > w { 1 } else { 0 },
                }
            }
            FillFrom::East | FillFrom::North | FillFrom::West | FillFrom::South => {
                // Count tiles on horizontal and vertical axes
                match amount {
                    FillAmount::Least => if steps <= w/2 { 0 } else { 1 },
                    FillAmount::NextLeast => if steps <= 3*w/2 { 0 } else { 1 },
                    FillAmount::FullEven => {
                        if steps <= 2*w+w/2 { 0 }
                        else {
                            // across center, width steps per tile, subtract Least and NextLeast tiles.
                            // finally add one (to round up) then divide by 2 to get just even
                            ((steps - w/2)/w - 2 + 1)/2
                        }
                    }
                    FillAmount::FullOdd => {
                        if steps <= 3*w+w/2 { 0 }
                        else {
                            // width/2 steps to get out of center, width steps per tile, subtract Least and NextLeast tiles.
                            // Add one to round up and finally divide by 2 to get just odd
                            ((steps - w/2)/w - 2)/2
                        }
                    }
                }
            }
            FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => {
                // Count tiles on triangular regions
                match amount {
                    FillAmount::Least => if steps <= w { 0 } else {
                        // These are the leading edge of the diagonals
                        (steps-1)/w
                    }
                    FillAmount::NextLeast => if steps <= 2*w { 0 } else {
                        // These are just behind the leading edge of the diagonals
                        (steps - (1+w))/w
                    }
                    FillAmount::FullEven => if steps <= 3*w { 0 } else {
                        // These are the bulk of the triangular area, with even step counts
                        let diags = (steps - (1+2*w))/w;
                        let even_diags = (diags+1)/2;
                        let even_tiles = even_diags * even_diags;
                        even_tiles
                    }
                    FillAmount::FullOdd => if steps <= 3*w { 0 } else {
                        // These are the bulk of the triangular area, with odd step counts
                        let diags = (steps - (1+2*w))/w;
                        let odd_diags = diags/2;
                        let odd_tiles = odd_diags * (odd_diags + 1);
                        odd_tiles
                    }

                }
            }
        }
    }

    fn count_for(start: &FillFrom, amount: &FillAmount, steps: usize, input: &Input) -> usize {
        let w = input.width;

        let sim_start = match start {
            FillFrom::NorthWest => (0, 0),
            FillFrom::North => ((w/2) as isize, 0),
            FillFrom::NorthEast => ((w-1) as isize, 0),
            FillFrom::West => (0, (w/2) as isize),
            FillFrom::Center => ((w/2) as isize, (w/2) as isize),
            FillFrom::East => ((w-1) as isize, (w/2) as isize),
            FillFrom::SouthWest => (0, (w-1) as isize),
            FillFrom::South => ((w/2) as isize, (w-1) as isize),
            FillFrom::SouthEast => ((w-1) as isize, (w-1) as isize),
        };

        let sim_steps = match amount {
            FillAmount::Least => {
                match start {
                    FillFrom::Center => steps,
                    FillFrom::North | FillFrom::East | FillFrom::South | FillFrom::West => (steps - w/2 - 1) % w,
                    FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => (steps - w - 1) % w
                }
            }
            FillAmount::NextLeast => {
                match start {
                    FillFrom::Center => panic!("Center as NextLeast shouldn't happen."),  // Shouldn't happen
                    FillFrom::North | FillFrom::East | FillFrom::South | FillFrom::West => (steps - w/2 - 1) % w + w,
                    FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => (steps - w - 1) % w + w
                }
            }
            FillAmount::FullEven => {
                let for_even = match start {
                    FillFrom::Center => w+1,
                    FillFrom::North | FillFrom::East | FillFrom::South | FillFrom::West => w+w/2,
                    FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => 2*w,
                };
                if steps & 1 == 0 { for_even } else { for_even + 1 }
            }
            FillAmount::FullOdd => {
                let for_odd = match start {
                    FillFrom::Center => panic!("Center as FullOdd shouldn't happen."),  // Shouldn't happen
                    FillFrom::North | FillFrom::East | FillFrom::South | FillFrom::West =>w+w/2,
                    FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => 2*w
                };
                if steps & 1 == 1 { for_odd } else { for_odd + 1 }
            }
        };
        
        let count_for = Day21::num_by_steps(sim_steps, input, &sim_start, false);
        // println!("  count_for(start:{start:?}, amount:{amount:?}, steps:{sim_steps:?}) -> {count_for}");

        count_for
    }

    fn num_by_tile_analysis(steps: usize, input: &Input) -> usize {
        let mut sum = 0;
        
        for start in FillFrom::iter() {
            for amount in FillAmount::iter() {
                let tiles = Self::num_of(&start, &amount, steps, input);       // how many tiles fit this start/amount
                if tiles != 0 {
                    let count = Self::count_for(&start, &amount, steps, input);    // how many places reached start/amount
                    // println!("{start:16?} {amount:16?} : {tiles} tiles of {count} places -> {}", tiles*count);
                    sum += tiles * count;
                }
            }
        }

        sum
    }

    fn num_by_steps(steps: usize, input: &Input, start: &(isize, isize), infinite: bool) -> usize {
        // We're going to do let 'current' and 'next' swap between these maps.
        let mut set1: HashSet<(isize, isize)> = HashSet::new();
        let mut set2: HashSet<(isize, isize)> = HashSet::new();

        let current = &mut set1;
        let next = &mut set2;

        // start position is the only occupied space, initially.
        current.insert(*start);  

        // println!("Doing num_by_steps: steps: {steps}, start:{start:?}, infinite:{infinite:?}");

        for _ in 0..steps {
            // clear out occupancy of next before we start
            next.clear();

            for (x, y) in current.iter() {
                // Check all neighbors.  If off, turn them on for next iteration.
                for (nx, ny) in [((*x+1), *y), (*x, (*y+1)), ((*x-1), *y), (*x, (*y-1))] {
                    if !infinite {
                        if (nx < 0) || (nx >= input.width as isize) || (ny < 0) || (ny >= input.height as isize) {
                            // This is outside the main block
                            continue;
                        }
                    }

                    let check_x = nx.rem_euclid(input.width as isize);
                    let check_y = ny.rem_euclid(input.height as isize);

                    // if (nx, ny) is in garden set put it in the next cycle
                    if input.garden.contains(&(check_x, check_y)) {
                        next.insert((nx, ny));
                    }
                }                
            }

            // set occupied to next
            std::mem::swap(current, next);
        }

        let num_occupied = current.len();

        num_occupied
    
    }

}

impl<'a> Day for Day21<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self.input_filename);

        let occupied = Day21::num_by_steps(64, &input, &input.start, false);

        Answer::Numeric(occupied)
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self.input_filename);
        let answer = Day21::num_by_tile_analysis(26501365, &input);

        Answer::Numeric(answer)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::day21::{Day21, Input, FillFrom, FillAmount};

    #[test]
    fn test_input() {
        let input = Input::read("examples/day21_example1.txt");
        assert_eq!(input.width, 11);
        assert_eq!(input.height, 11);
        assert_eq!(input.start, (5, 5));
        assert!(!input.garden.contains(&(1, 2)));
        assert!(input.garden.contains(&(0, 2)));
    }

    
    #[test]
    fn check_real_input() {
        let input = Input::read("data_aoc2023/day21.txt");
        assert_eq!(input.width, 131);
        assert_eq!(input.height, 131);
        assert_eq!(input.start, (65, 65));
    }

    #[test]
    fn test_num_by_steps() {
        let input = Input::read("examples/day21_example1.txt");

        let cases= [
            (6, 16), 
            (10, 50), 
            (50, 1594),
            (100, 6536), 
            (500, 167004), 
            //(1000, 668697), 
            // (5000, 16733044)
            ];
        for (steps, expected) in &cases {
            let reach = Day21::num_by_steps(*steps, &input, &input.start, true);
            assert_eq!(reach, *expected);
        }
    }

    fn steps_from(input: &Input, start: (isize, isize), end: (isize, isize)) -> usize {
        // We're going to do let 'current' and 'next' swap between these maps.
        let mut set1: HashSet<(isize, isize)> = HashSet::new();
        let mut set2: HashSet<(isize, isize)> = HashSet::new();

        let current = &mut set1;
        let next = &mut set2;

        // start position is the only occupied space, initially.
        current.insert(start);  

        let mut steps = 0;

        loop {
            // clear out occupancy of next before we start
            next.clear();

            // Check to see if we're done.
            if current.contains(&end) {
                break;
            }

            for (x, y) in current.iter() {
                // Check all neighbors.  If off, turn them on for next iteration.
                for (nx, ny) in [((*x+1), *y), (*x, (*y+1)), ((*x-1), *y), (*x, (*y-1))] {
                    let check_x = nx.rem_euclid(input.width as isize);
                    let check_y = ny.rem_euclid(input.height as isize);

                    // if (nx, ny) is in garden set put it in the next cycle
                    if input.garden.contains(&(check_x, check_y)) {
                        next.insert((nx, ny));
                    }
                }                
            }

            // set occupied to next
            std::mem::swap(current, next);

            steps += 1;
        }

        steps
    }

    #[test]
    fn test_steps_from() {
        let input = Input::read("examples/day21_example1.txt");

        assert_eq!(steps_from(&input, (5, 5), (5, 4)), 1);
        assert_eq!(steps_from(&input, (5, 5), (5, 3)), 2);
        assert_eq!(steps_from(&input, (5, 5), (7, 4)), 5);
        assert_eq!(steps_from(&input, (5, 5), (0, 5)), 7);
        assert_eq!(steps_from(&input, (5, 5), (-1, 5)), 8);
    }


    #[test]
    fn test_steps_from_real() {
        let input = Input::read("data_aoc2023/day21.txt");
        let w = input.width as isize;

        // From start to N, S, E, W on adjacent tile.
        assert_eq!(steps_from(&input, (w/2, w/2), (-1, w/2)), (w/2+1) as usize);
        assert_eq!(steps_from(&input, (w/2, w/2), (w, w/2)), (w/2+1) as usize);
        assert_eq!(steps_from(&input, (w/2, w/2), (w/2, -1)), (w/2+1) as usize);
        assert_eq!(steps_from(&input, (w/2, w/2), (w/2, w as isize)), (w/2+1) as usize);

        // from start to NE, SE, NW, SW on adjacent tile.
        assert_eq!(steps_from(&input, (w/2, w/2), (-1, -1)), (w+1) as usize);
        assert_eq!(steps_from(&input, (w/2, w/2), (w as isize, w)), (w+1) as usize);
        assert_eq!(steps_from(&input, (w/2, w/2), (w, -1)), (w+1) as usize);
        assert_eq!(steps_from(&input, (w/2, w/2), (-1, w)), (w+1) as usize);

        // Horizontal/Vert from edge to next tile's edge.
        assert_eq!(steps_from(&input, (0, 0), (0, w)), w as usize);
        assert_eq!(steps_from(&input, (0, 0), (w, 0)), w as usize);

        // Diagonal from corner to opposite corner, adjacent tile.
        assert_eq!(steps_from(&input, (0, 0), (w, w)), (2*w) as usize);
        assert_eq!(steps_from(&input, (0, (w-1)), (w, -1)), (2*w) as usize);
    }    

    #[test]
    fn test_num_of_center() {
        let input = Input::read("data_aoc2023/day21.txt");
        let w = input.width;

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 1, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 1, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 1, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 1, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, w/2, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, w/2, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, w/2, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, w/2, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, w/2+1, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, w/2+1, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, w/2+1, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, w/2+1, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 2, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 2, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 2, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 2, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, w+1, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, w+1, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, w+1, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, w+1, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, w+2, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, w+2, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, w+2, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, w+2, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 26501364, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 26501364, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 26501364, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 26501364, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 26501365, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 26501365, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 26501365, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 26501365, &input), 0);
    }


    #[test]
    fn test_num_of_nesw() {
        let input = Input::read("data_aoc2023/day21.txt");
        let w = input.width;

        for seed in &[FillFrom::North, FillFrom::South, FillFrom::East, FillFrom::West] {
            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 1, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2+1, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2+w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2+w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2+w, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2+w, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2+w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2+w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2+w+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2+w+1, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2+2*w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2+2*w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2+2*w, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2+2*w, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2+2*w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2+2*w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2+2*w+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2+2*w+1, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2+3*w, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2+3*w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2+3*w, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2+3*w, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2+3*w+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2+3*w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2+3*w+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2+w+1, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2+4*w, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2+4*w, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2+4*w, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2+4*w, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w/2+4*w+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w/2+4*w+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w/2+4*w+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w/2+4*w+1, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 26501364, &input), 101149);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 26501364, &input), 101148);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 26501364, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 26501364, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 26501365, &input), 101149);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 26501365, &input), 101149);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 26501365, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 26501365, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 26501365, &input), 101149);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 26501365, &input), 101149);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 26501365, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 26501365, &input), 1);
        }
    }

    #[test]
    fn test_num_of_diags() {
        let input = Input::read("data_aoc2023/day21.txt");
        let w = input.width;

        for seed in &[FillFrom::NorthEast, FillFrom::SouthEast, FillFrom::NorthWest, FillFrom::SouthWest] {
            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 1, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, w+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, w+1, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 2*w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 2*w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 2*w, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 2*w, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 2*w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 2*w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 2*w+1, &input), 2);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 2*w+1, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 3*w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 3*w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 3*w, &input), 2);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 3*w, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 3*w+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 3*w+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 3*w+1, &input), 3);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 3*w+1, &input), 2);
            
            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 4*w, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 4*w, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 4*w, &input), 3);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 4*w, &input), 2);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 26501365, &input), 10231120201);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 26501365, &input), 10231221350);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 26501365, &input), 202300);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 26501365, &input), 202299);
        }
    }

    #[test]
    fn test_count_for_center() {
        let input = Input::read("data_aoc2023/day21.txt");
        let w = input.width;

        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::Least, 1, &input), 4);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::Least, w/2, &input), 3821);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::Least, w/2+1, &input), 3984);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::Least, w, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::FullEven, w+1, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::FullEven, w+2, &input), 7556);
    }

    #[test]
    fn test_count_for_nsew() {
        let input = Input::read("data_aoc2023/day21.txt");
        let w = input.width;

        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::Least, w/2+1, &input), 1);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::Least, w/2+2, &input), 3);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::Least, w/2+w-1, &input), 5656);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::Least, w/2+w, &input), 5692);

        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::NextLeast, w/2+w+1, &input), 5786);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::NextLeast, w/2+2*w, &input), 7602);

        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullEven, 3*w+w/2, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullEven, 3*w+w/2+1, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullEven, 23*w+w/2, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullEven, 23*w+w/2+1, &input), 7602);

        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullOdd, 3*w+w/2, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullOdd, 3*w+w/2+1, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullOdd, 23*w+w/2, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullOdd, 23*w+w/2+1, &input), 7556);

        // ------------------------------------------------------------------------------------------------

        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::Least, w/2+1, &input), 1);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::Least, w/2+2, &input), 3);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::Least, w/2+w-1, &input), 5667);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::Least, w/2+w, &input), 5705);

        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::NextLeast, w/2+1+w, &input), 5797);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::NextLeast, w/2+1+2*w-1, &input), 7602);

        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullEven, 3*w+w/2, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullEven, 3*w+w/2+1, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullEven, 23*w+w/2, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullEven, 23*w+w/2+1, &input), 7602);

        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullOdd, 3*w+w/2, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullOdd, 3*w+w/2+1, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullOdd, 23*w+w/2, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullOdd, 23*w+w/2+1, &input), 7556);

        // -----------------------------------------------------------------------------------------------

        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullEven, 3*w+w/2, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullEven, 3*w+w/2+1, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullEven, 23*w+w/2, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullEven, 23*w+w/2+1, &input), 7602);

        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullOdd, 3*w+w/2, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullOdd, 3*w+w/2+1, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullOdd, 23*w+w/2, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullOdd, 23*w+w/2+1, &input), 7556);

        // -----------------------------------------------------------------------------------------------

        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullEven, 3*w+w/2, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullEven, 3*w+w/2+1, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullEven, 23*w+w/2, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullEven, 23*w+w/2+1, &input), 7602);

        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullOdd, 3*w+w/2, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullOdd, 3*w+w/2+1, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullOdd, 23*w+w/2, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullOdd, 23*w+w/2+1, &input), 7556);

    }

    #[test]
    fn test_regression() {
        // Exploring whether the total can be found fitting a quadratic. (Not fully determined.)
        // let input = Input::read("data_aoc2023/day21.txt");
        let input = Input::read("examples/day21_example1.txt");

        let a = Day21::num_by_steps(input.start.0 as usize, &input, &input.start, true);
        let b = Day21::num_by_steps(input.start.0 as usize + 6*input.width, &input, &input.start, true);
        let c = Day21::num_by_steps(input.start.0 as usize + 8*input.width, &input, &input.start, true);

        // println!("a: {a}, b: {b}, c:{c}");
        
        assert_eq!(a, 13);
        assert_eq!(b, 3282);
        assert_eq!(c, 5684);
    }

    #[test]
    fn test_num_by_tile_analysis() {
        let input = Input::read("data_aoc2023/day21.txt");

        // 465262978619852 is too low.
        // 620348631940729 is too high.
        let answer = Day21::num_by_tile_analysis(26501365, &input);
        assert!(answer > 465262978619852);
        assert!(answer < 620348631940729);
        assert_eq!(answer, 620348631910321);
    }
}
