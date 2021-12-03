use crate::common::{iAoC, Error};
use std::str::FromStr;

enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
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
        Ok(match first {
            "forward" => Command::Forward(steps),
            "up" => Command::Up(steps),
            "down" => Command::Down(steps),
            _ => return Err(Error::new("unknown command")),
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

fn read_commands(input: &str) -> Result<Vec<Command>, Error> {
    match input.lines().map(|line| Command::from_str(line)).collect() {
        Err(err) => Err(Error::new(err.to_string())),
        Ok(coll) => Ok(coll),
    }
}

pub fn solve_a(input: &str) -> Result<iAoC, Error> {
    let commands = read_commands(input)?;
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };
    for command in commands {
        match command {
            Command::Forward(steps) => position.horizontal += steps,
            Command::Down(steps) => position.depth += steps,
            Command::Up(steps) => position.depth -= steps,
        }
    }
    let result = position.horizontal * position.depth;
    Ok(result as u64)
}

pub fn solve_b(input: &str) -> Result<iAoC, Error> {
    let commands = read_commands(input)?;
    let mut position = AimPosition {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for command in commands {
        match command {
            Command::Forward(steps) => {
                position.horizontal += steps;
                position.depth += position.aim * steps
            }
            Command::Down(steps) => position.aim += steps,
            Command::Up(steps) => position.aim -= steps,
        }
    }
    let result = position.horizontal * position.depth;
    Ok(result as iAoC)
}
