mod day;
mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;


use day::{Day, Answer};
use day0::Day0;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;



fn report_day(day: &dyn Day, day_no: usize) {
    
    let ans1 = day.part1();
    let msg1 = match ans1 {
        Answer::None => String::from("No Answer"),
        Answer::Numeric(n) => format!("{n}"),
        Answer::String(s) => format!("{s}"),
    };

    let ans2 = day.part2();
    let msg2 = match ans2 {
        Answer::None => String::from("No Answer"),
        Answer::Numeric(n) => format!("{n}"),
        Answer::String(s) => format!("{s}"),
    };
    println!("Day {day_no:2}: {msg1:>16} {msg2:>16}");
}

fn main() {
    println!("Advent of Code 2023!\n");

    let days: [&mut dyn Day; 26] = [
        &mut Day0::new("data_aoc2023/day0.txt"),  // Placeholder
        &mut Day1::new("data_aoc2023/day1.txt"),  // Dec 1
        &mut Day2::new("data_aoc2023/day2.txt"),
        &mut Day3::new("data_aoc2023/day3.txt"),
        &mut Day4::new("data_aoc2023/day4.txt"),
        &mut Day5::new("data_aoc2023/day5.txt"),  // Dec 5
        &mut Day6::new("data_aoc2023/day6.txt"),
        &mut Day7::new("data_aoc2023/day7.txt"),
        &mut Day8::new("data_aoc2023/day8.txt"),
        &mut Day9::new("data_aoc2023/day9.txt"),
        &mut Day10::new("data_aoc2023/day10.txt"),  // Dec 10
        &mut Day11::new("data_aoc2023/day11.txt"),
        &mut Day12::new("data_aoc2023/day12.txt"),
        &mut Day13::new("data_aoc2023/day13.txt"),
        &mut Day14::new("data_aoc2023/day14.txt"),
        &mut Day15::new("data_aoc2023/day15.txt"),  // Dec 15
        &mut Day16::new("data_aoc2023/day16.txt"),
        &mut Day17::new("data_aoc2023/day17.txt"),
        &mut Day18::new("data_aoc2023/day18.txt"),
        &mut Day19::new("data_aoc2023/day19.txt"),
        &mut Day20::new("data_aoc2023/day20.txt"),  // Dec 20
        &mut Day21::new("data_aoc2023/day21.txt"),
        &mut Day22::new("data_aoc2023/day22.txt"),
        &mut Day23::new("data_aoc2023/day23.txt"),
        &mut Day24::new("data_aoc2023/day24.txt"),
        &mut Day25::new("data_aoc2023/day25.txt"),  // Dec 25
    ];

    let target_day = 0;

    match target_day {
        0 => {
            // report all days
            println!("{:7} {:>16} {:>16}", "", "Part 1", "Part 2");
            for day_no in 1..=25 {
                report_day(days[day_no], day_no);
            }
        }
        1..=25 => {
            // report a specific day
            report_day(days[target_day], target_day);
        }
        _ => {
            // invalid day 
            println!("Day {target_day} is invalid.\n");
        }
    }
    println!();

}

#[cfg(test)]
mod test {
    use crate::day::{Day, Answer};
    use crate::{Day1, Day2, Day3};

    #[test]
    fn test_day1_part1() {
        let d: Day1 = Day1::new("data_aoc2023/day1.txt");
        assert_eq!(d.part1(), Answer::Numeric(55029));
    }

    #[test]
    fn test_day1_part2() {
        let d: Day1 = Day1::new("data_aoc2023/day1.txt");
        assert_eq!(d.part2(), Answer::Numeric(55686));  // Not 55680
    }

    #[test]
    fn test_day2_part1() {
        let d = Day2::new("data_aoc2023/day2.txt");
        assert_eq!(d.part1(), Answer::Numeric(1853));
    }

    #[test]
    fn test_day2_part2() {
        let d = Day2::new("data_aoc2023/day2.txt");
        assert_eq!(d.part2(), Answer::Numeric(72706));
    }

    #[test]
    fn test_day3_part1() {
        let d = Day3::new("data_aoc2023/day3.txt");
        assert_ne!(d.part1(), Answer::Numeric(508789));
        assert_eq!(d.part1(), Answer::Numeric(525911));
    }

    #[test]
    fn test_day3_part2() {
        let d = Day3::new("data_aoc2023/day3.txt");
        assert_eq!(d.part2(), Answer::Numeric(75805607));
    }
}
