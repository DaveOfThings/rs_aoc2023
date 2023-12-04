use std::fs::File;

use crate::day::{Day, Answer, LineBasedInput};

struct Play {
    red: usize,
    green: usize,
    blue: usize,
}

impl Play {
    fn possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

struct Game {
    game_no: usize,
    plays: Vec<Play>
}

impl Game {
    fn possible(&self) -> bool {
        self.plays.iter().fold(true, |summary, play| summary && play.possible())
    }

    fn power(&self) -> usize {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for play in &self.plays {
            if play.red > min_red {
                min_red = play.red;
            }
            if play.green > min_green {
                min_green = play.green;
            }
            if play.blue > min_blue {
                min_blue = play.blue;
            }
        }

        min_red * min_green * min_blue
    }
}

struct Input {
    games: Vec<Game>,
}

pub struct Day2<'a>{
    input_filename: &'a str,
}

impl<'a> Day2<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename}
    }

    fn read_input(&self, _part2: bool) -> Input {
        let infile = File::open(&self.input_filename).expect("Failed to open puzzle input.");
        let records = self.process(infile, false);
        
        Input { games: records }
    }
}

impl<'a> LineBasedInput<Game> for Day2<'a> {

    fn parse_line(line: &str, _part2: bool) -> Option<Game> {
        // Process lines that look like this:
        // "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"

        // ("Game 5", "6 rd, 1 blue ...")
        let split1: Vec<&str> = line.split(":").collect::<Vec<&str>>(); 

        let game_no = split1[0].split(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

        // "6 red, 1 blue ...", "2 blue, 1 red, ...";
        let split2 = split1[1].split(";"); 

        let mut plays: Vec<Play> = Vec::new();
        for play_str in split2 {
            let mut play = Play { red: 0, green: 0, blue: 0 };

            // Split into "6 red"
            for show_str in play_str.split(",") {
                // "6", "red"
                let split3 = show_str.strip_prefix(" ")?.split(" ").collect::<Vec<&str>>(); // "6", "red"
                let n = split3[0].parse::<usize>().unwrap();
                let color = split3[1];

                match color {
                    "red" => play.red = n,
                    "green" => play.green = n,
                    "blue" => play.blue = n,
                    _ => panic!(),
                }
            }

            plays.push(play);
        }


        Some(Game {game_no, plays} )
    }
}

impl<'a> Day for Day2<'a> {


    fn part1(&self) -> Answer {
        let input = self.read_input(false);

        // Sum all the game numbers where the plays are actually possible.
        let sum = 
            input.games.iter().filter(|game| game.possible()).map(|game| game.game_no).sum();

        Answer::Numeric(sum)
    }

    fn part2(&self) -> Answer {
        let input = self.read_input(true);

        let sum = 
            input.games.iter().map(|game| game.power()).sum();

        Answer::Numeric(sum)

    }
}

#[cfg(test)]
mod tests {
    use crate::{Day, Answer, Day2};
    use crate::day2::Play;

    #[test]
    fn test_input_p1() {
        let d = Day2::new("examples/day2_example1.txt");
        let input = d.read_input(false);

        assert_eq!(input.games.len(), 5);
    }

    #[test]
    fn test_input_p2() {
        let d = Day2::new("examples/day2_example1.txt");
        let input = d.read_input(true);

        assert_eq!(input.games.len(), 5);
    }

    #[test]
    fn test_plays() {
        let p1 = Play {red: 12, green: 13, blue: 14};
        assert!(p1.possible());

        let p2 = Play {red: 13, green: 13, blue: 14};
        assert!(!p2.possible());

        let p3 = Play {red: 12, green: 14, blue: 14};
        assert!(!p3.possible());

        let p4 = Play {red: 12, green: 13, blue: 15};
        assert!(!p4.possible());
    }

    #[test]
    fn test_part1() {
        let d = Day2::new("examples/day2_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(8));
    }

    #[test]
    fn test_power() {
        let d = Day2::new("examples/day2_example1.txt");
        let input = d.read_input(true);

        assert_eq!(input.games[0].power(), 48);
        assert_eq!(input.games[1].power(), 12);
        assert_eq!(input.games[2].power(), 1560);
        assert_eq!(input.games[3].power(), 630);
        assert_eq!(input.games[4].power(), 36);
    }


    #[test]
    fn test_part2() {
        let d = Day2::new("examples/day2_example1.txt");
        assert_eq!(d.part2(), Answer::Numeric(2286));
    }
}
