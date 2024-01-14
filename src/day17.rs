use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use petgraph::graphmap::DiGraphMap;
use petgraph::algo::dijkstra;

use crate::day::{Day, Answer};




#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Right,
        Direction::Left,
    ];
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct NodeId {
    row: usize,
    col: usize,
    dir: Direction,
    momentum: usize,
}

struct Input {
    grid: Vec<Vec<usize>>,
}

impl Input {
    const MAX_MOMENTUM: usize = 3;
    const MAX_ULTRA_MOMENTUM: usize = 10;

    pub fn read(filename: &str) -> Input {
        let mut grid: Vec<Vec<usize>> = Vec::new();

        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let mut grid_line: Vec<usize> = Vec::new();

            for c in line.unwrap().chars() {
                grid_line.push(c.to_digit(10).unwrap() as usize);
            }

            grid.push(grid_line);
        }

        Input { grid }
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn cols(&self) -> usize {
        self.grid[0].len()
    }

    // Determines if move_dir is a valid move under the circumstances.
    // If so, returns Some(new_row, new_col, new_momentum)
    // Otherwise, returns None
    fn valid_move(&self, row: usize, col: usize, 
                 dir: Direction, momentum: usize, 
                 move_dir: Direction) -> Option<(usize, usize, usize)> 
                 {
        let opposite: HashMap<Direction, Direction> = HashMap::from([
            (Direction::Up, Direction::Down),
            (Direction::Down, Direction::Up),
            (Direction::Right, Direction::Left),
            (Direction::Left, Direction::Right),
        ]);

        let dy_dx: HashMap<Direction, (isize, isize)> = HashMap::from([
            (Direction::Up, (-1, 0)),
            (Direction::Down, (1, 0)),
            (Direction::Right, (0, 1)),
            (Direction::Left, (0, -1)),
        ]);

        if (move_dir == opposite[&dir]) && (momentum > 0) { 
            // Can't reverse
            return None;
        };

        let new_momentum = if (move_dir == dir) && (momentum > 0) {
            // increase momentum since we are moving in the same dir
            momentum + 1
        }
        else {
            // reset momentum to 1
            1
        };

        // Limit momentum build up
        if new_momentum > Self::MAX_MOMENTUM { 
            // continuing to move in same dir, momentum would exceed limit
            return None;
        };

        let (delta_row, delta_col) = dy_dx[&move_dir];
        let new_row = row as isize + delta_row;
        let new_col = col as isize + delta_col;
        if (new_row < 0) || (new_row as usize >= self.rows()) ||
           (new_col < 0) || (new_col as usize >= self.cols()) {
            // We went off the edge of the map
            return None;
        }
        let new_row = new_row as usize;
        let new_col = new_col as usize;

        Some((new_row, new_col, new_momentum))
    }

    // Determines if move_dir is a valid move under the circumstances.
    // If so, returns Some(new_row, new_col, new_momentum)
    // Otherwise, returns None
    fn valid_ultra_move(&self, row: usize, col: usize, 
        dir: Direction, momentum: usize, 
        move_dir: Direction) -> Option<(usize, usize, usize)> 
        {
        let opposite: HashMap<Direction, Direction> = HashMap::from([
        (Direction::Up, Direction::Down),
        (Direction::Down, Direction::Up),
        (Direction::Right, Direction::Left),
        (Direction::Left, Direction::Right),
        ]);

        let dy_dx: HashMap<Direction, (isize, isize)> = HashMap::from([
        (Direction::Up, (-1, 0)),
        (Direction::Down, (1, 0)),
        (Direction::Right, (0, 1)),
        (Direction::Left, (0, -1)),
        ]);

        // Can't reverse (Is this a constraint, still?)
        if (move_dir == opposite[&dir]) && (momentum > 0) { 
            // Can't reverse
            return None;
        };

        // Can't change direction until momentum is 4 or greater
        if (move_dir != dir) && (momentum > 0) && (momentum < 4) {
            return None;
        }

        // Can't let momentum go above 10
        let new_momentum = if (move_dir == dir) && (momentum > 0) {
            // increase momentum since we are moving in the same dir
            momentum + 1
        }
        else {
            // reset momentum to 1
            1
        };
        if new_momentum > Self::MAX_ULTRA_MOMENTUM { 
            // exceeded momentum limit
            return None;
        };

        let (delta_row, delta_col) = dy_dx[&move_dir];
        let new_row = row as isize + delta_row;
        let new_col = col as isize + delta_col;
        if (new_row < 0) || (new_row as usize >= self.rows()) ||
           (new_col < 0) || (new_col as usize >= self.cols()) {
            // We went off the edge of the map
            return None;
        }
        let new_row = new_row as usize;
        let new_col = new_col as usize;

        Some((new_row, new_col, new_momentum))
    }

    fn to_graph(&self, ultra: bool) -> DiGraphMap<NodeId, usize> {


        let mut g: DiGraphMap<NodeId, usize> = DiGraphMap::new();

        /*
        // add nodes (Not neccessary as adding edges does it automatically!)
        // 13 nodes per board position: one for each combination of 
        // direction and momentum.
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                for dir in Direction::DIRECTIONS {
                    for momentum in 0..=3 {
                        // Add a node to the graph for this position/momentum combo
                        let node = NodeId { row, col, dir, momentum };
                        g.add_node(node);
                    }
                }
            }
        }
        */

        // add edges
        // From every position on the board, add edges from nodes corresponding 
        // to all the momentum states at that position.  Don't create edges that
        // reverse momentum and don't create edges that increase momentum past
        // the maximum.  When continuing in same direction, increase momentum
        // but when changing direction, reset momentum to 1.
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                for dir in Direction::DIRECTIONS {
                    for momentum in 0..=10 {
                        // Starting from the state with this position, momentum
                        // Test whether we can move in each direction and add
                        // an appropriate destination state if so.
                        for move_dir in Direction::DIRECTIONS {
                            let valid_move = if ultra {
                                self.valid_ultra_move(row, col, dir, momentum, move_dir)
                            }
                            else {
                                self.valid_move(row, col, dir, momentum, move_dir)
                            };

                            if let Some((new_row, new_col, new_momentum)) = valid_move {
                                // Everything is good for a transition from (row, col, dir, momentum) to 
                                // (new_row, new_col, move_dir, new_momentum) with cost self.grid[new_row][new_col];
                                // Add an edge to the graph for this move.
                                let from = NodeId { row, col, dir, momentum };
                                let to = NodeId { row:new_row, col: new_col, dir: move_dir, momentum: new_momentum };
                                g.add_edge(from, to, self.grid[new_col][new_row]);
                            }
                        }
                    }
                }
            }
        }

        g
    }

    pub fn least_heat(&self, ultra: bool) -> usize {
        // get the graph representation of possible movements.
        let g = self.to_graph(ultra);
        
        println!("We have a directed graph and that's exciting.");

        // Do Dijkstra to get cost (heat loss) to every node.
        let start = NodeId{ row: 0, col: 0, dir:Direction::Right, momentum: 0 };
        let distances = dijkstra(
            &g,                   // graph
            start,                     // start
            None,                 // goal
            |edge| *edge.2  // edge cost
        );
        println!("We have dijkstra results and that's more exciting.");

        // The final position corresponds to several graph nodes
        // (because there are many momentum states)
        // Pick the best one.
        let min_dist = distances.iter()
            .filter(|(nodeid, _d)| (nodeid.row == self.rows()-1) && (nodeid.col == self.cols()-1))
            .filter(|(nodeid, _d)| !ultra || (nodeid.momentum >= 4))
            .map(|(_nodeid, d)| d)
            .min()
            .unwrap();

        // Bob's your uncle.
        *min_dist
    }

}

pub struct Day17<'a> {
    input_filename: &'a str,
}

impl<'a> Day17<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }
}

impl<'a> Day for Day17<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self.input_filename);
        Answer::Numeric(input.least_heat(false))
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self.input_filename);
        Answer::Numeric(input.least_heat(true))
    }
}

#[cfg(test)]
mod tests {
    use crate::day17::Input;

    #[test]
    fn test_input() {
        let input = Input::read("examples/day17_example1.txt");
        assert_eq!(input.rows(), 13);
        assert_eq!(input.cols(), 13);
    }

    #[test]
    fn test_least_heat() {
        let input = Input::read("examples/day17_example1.txt");
        assert_eq!(input.least_heat(false), 102);
    }


    #[test]
    fn test_ultra_least_heat() {
        let input = Input::read("examples/day17_example1.txt");
        assert_eq!(input.least_heat(true), 94);
    }
}