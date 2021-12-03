use crate::common::{iAoC, Error, Solver};
use std::str::FromStr;

pub struct Day2 {}

struct Command {
    pub direction: String,
    pub steps: i64,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let (first, second) = match command.split_once(" ") {
            None => return Err(Error::new("no space detected")),
            Some(split) => split,
        };
        let steps = match second.parse::<i64>() {
            Err(err) => return Err(Error::new(err.to_string())),
            Ok(num) => num,
        };
        Ok(Command {
            direction: first.to_owned(),
            steps,
        })
    }
}

struct Position {
    pub horizontal: i64,
    pub depth: i64,
}

struct AimPosition {
    pub horizontal: i64,
    pub depth: i64,
    pub aim: i64,
}

impl Day2 {
    fn read_commands(input: &str) -> Result<Vec<Command>, Error> {
        match input.lines().map(|line| Command::from_str(line)).collect() {
            Err(err) => Err(Error::new(err.to_string())),
            Ok(coll) => Ok(coll),
        }
    }
}

impl Solver for Day2 {
    fn solve_a(input: &str) -> Result<iAoC, Error> {
        let commands = Day2::read_commands(input)?;
        let mut position = Position {
            horizontal: 0,
            depth: 0,
        };
        for command in commands {
            match command.direction.as_ref() {
                "forward" => position.horizontal += command.steps,
                "down" => position.depth += command.steps,
                "up" => position.depth -= command.steps,
                _ => return Err(Error::new("invalid direction")),
            }
        }
        let result: i64 = position.horizontal * position.depth;
        Ok(result)
    }
    fn solve_b(input: &str) -> Result<iAoC, Error> {
        let commands = Day2::read_commands(input)?;
        let mut position = AimPosition {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };
        for command in commands {
            match command.direction.as_ref() {
                "forward" => {
                    position.horizontal += command.steps;
                    position.depth += position.aim * command.steps
                }
                "down" => position.aim += command.steps,
                "up" => position.aim -= command.steps,
                _ => return Err(Error::new("invalid direction")),
            }
        }
        let result: i64 = position.horizontal * position.depth;
        Ok(result)
    }
}
