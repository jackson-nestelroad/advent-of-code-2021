use super::*;
use crate::common::{iAoC, Error, Solver, SolverFn};
use crate::program::{ProgramArgs, SolutionPart};
use std::fs;
use std::time::{Duration, Instant};

const SOLVERS: [[SolverFn; 2]; 2] = [
    [Day1::solve_a, Day1::solve_b],
    [Day2::solve_a, Day2::solve_b],
];

fn get_solver(args: &ProgramArgs) -> Result<SolverFn, Error> {
    if args.day() as usize > SOLVERS.len() {
        return Err(Error::new("day not implemented"));
    }

    let part_index: usize = match args.part() {
        SolutionPart::A => 0,
        SolutionPart::B => 1,
    };
    return Ok(SOLVERS[(args.day() - 1) as usize][part_index]);
}

pub struct Solution {
    solution: iAoC,
    time: Duration,
}

impl Solution {
    pub fn new(solution: iAoC, time: Duration) -> Self {
        Solution { solution, time }
    }

    pub fn solution(&self) -> iAoC {
        self.solution
    }

    pub fn time(&self) -> &Duration {
        &self.time
    }
}

pub fn solve(args: &ProgramArgs) -> Result<Solution, Error> {
    let solver = get_solver(args)?;
    let file_name = format!("input/{}.txt", args.day());
    let input = match fs::read_to_string(file_name) {
        Err(err) => return Err(Error::new(err.to_string())),
        Ok(input) => input,
    };
    let now = Instant::now();
    let solution = solver(&input)?;
    let then = now.elapsed();
    Ok(Solution::new(solution, then))
}
