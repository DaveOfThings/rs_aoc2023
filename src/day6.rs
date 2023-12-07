use std::{fs::File, io::BufReader, io::BufRead};
use regex::Regex;

use crate::day::{Day, Answer};

struct Event {
    event_time: usize,
    distance: usize,
}

impl Event {
    // return (min, max) times to win the event
    fn analyze(&self) -> (usize, usize) {
        // let t = time button pressed
        // then v = t, 
        //      tm, moving time = event_time - t.
        // distance = v * tm = t * (event_time - t) = -t^2 + event_time*t + 0
        // or -t^2 + event_time*t - distance = 0,
        // We have A = -1, B = event_time, C = -distance
        // t = 
        // distance = press_time * (self.time - press_time)
        // distance = -1 * press_time^2 + self.time * press_time + 0
        //  or 0 = Ax^2 + Bx + C where x is press_time, A = -1, B=self.time, C = -distance
        // 
        // t_min = (-B - sqrt(B^2-4*AC))/2A
        // t_max = (-B + sqrt(B^2-4*A*C))/2A
        // t_best = (t_max + t_min)/2

        let neg_b = -(self.event_time as f64);
        let rad2 = (self.event_time * self.event_time - 4*self.distance) as f64;
        let rad = rad2.sqrt();
        let denom = -2.0_f64;

        let t1 = (neg_b + rad)/denom;
        let t2 = (neg_b - rad)/denom;
        // println!("t1: {t1}, t2: {t2}");

        let mut t1: usize = t1.ceil() as usize;
        let mut t2: usize = t2.floor() as usize;

        // don't settle for a tie
        if t1 * (self.event_time - t1) == self.distance {
            t1 += 1;
        }
        if t2 * (self.event_time - t2) == self.distance {
            t2 -= 1;
        }

        (t1, t2)
    }

}

struct Input {
    events: Vec<Event>
}

impl Input {
    fn margin(&self) -> usize {
        let mut prod = 1;
        for e in &self.events {
            let (min, max) = e.analyze();
            prod *= max - min + 1;
        }

        prod
    }
}

pub struct Day6<'a> {
    input_filename: &'a str,
}

impl<'a> Day6<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }

    fn read_input(&self, _part2: bool) -> Input {
        let num_re = Regex::new("([\\d]+)").unwrap();
        
        let infile = File::open(&self.input_filename).expect("Failed to open puzzle input.");

        let mut reader = BufReader::new(infile);
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();

        // For part 2, concatenate all the digits
        if _part2 {
            let s2: String = 
                s.chars().filter(|c| c.is_digit(10)).collect();
            s = s2;
        }

        let times: Vec<usize> = num_re.captures_iter(&s)
            .map(|c| c[1].parse().unwrap())
            .collect();
        s.clear();
        reader.read_line(&mut s).unwrap();

        // For part 2, concatenate all the digits
        if _part2 {
            let s2: String = 
                s.chars().filter(|c| c.is_digit(10)).collect();
            s = s2;
        }

        let distances: Vec<usize> = num_re.captures_iter(&s)
            .map(|c| c[1].parse().unwrap())
            .collect();

        assert_eq!(times.len(), distances.len());

        let mut events: Vec<Event> = Vec::new();
        for n in 0..times.len() {
            events.push( Event {event_time: times[n], distance: distances[n]});
        }

        Input {events}
    }
}

impl<'a> Day for Day6<'a> {
    fn part1(&self) -> Answer {
        let input = self.read_input(false);

        Answer::Numeric(input.margin())
    }

    fn part2(&self) -> Answer {
        let input = self.read_input(true);

        Answer::Numeric(input.margin())
    }
}

#[cfg(test)]
mod tests {

    use crate::{Day, Answer, Day6};

    #[test]
    fn test_input_p1() {        
        let d = Day6::new("examples/day6_example1.txt");
        let input = d.read_input(false);

        assert_eq!(input.events.len(), 3);
    }

    #[test]
    fn test_analysis() {        
        let d = Day6::new("examples/day6_example1.txt");
        let input = d.read_input(false);

        assert_eq!(input.events[0].analyze(), (2, 5));
        assert_eq!(input.events[1].analyze(), (4, 11));
        assert_eq!(input.events[2].analyze(), (11, 19));
    }

    #[test]
    fn test_margin() {
        let d = Day6::new("examples/day6_example1.txt");
        let input = d.read_input(false);

        assert_eq!(input.margin(), 288);
    }

    #[test]
    fn test_margin2() {
        let d = Day6::new("examples/day6_example1.txt");
        let input = d.read_input(true);

        assert_eq!(input.margin(), 71503);
    }

    #[test]
    fn test_part1() {
        let d = Day6::new("examples/day6_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(288));
    }

    #[test]
    fn test_part2() {
        let d = Day6::new("examples/day6_example1.txt");

        assert_eq!(d.part2(), Answer::Numeric(71503));
    }
}
