use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use std::str::FromStr;

enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Command {
    type Err = AocError;

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let (first, second) = command
            .split_once(" ")
            .into_aoc_result_msg("no space detected")?;
        let steps = second.parse::<i64>().into_aoc_result()?;
        Ok(match first {
            "forward" => Command::Forward(steps),
            "up" => Command::Up(steps),
            "down" => Command::Down(steps),
            _ => return Err(AocError::new("unknown command")),
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

fn read_commands(input: &str) -> AocResult<Vec<Command>> {
    input
        .lines()
        .map(|line| Command::from_str(line))
        .collect::<Result<_, _>>()
        .into_aoc_result()
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
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

pub fn solve_b(input: &str) -> AocResult<iAoc> {
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
    Ok(result as iAoc)
}
