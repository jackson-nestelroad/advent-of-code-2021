use super::*;
use crate::common::{iAoc, AocError, AocResult, IntoAocResult, SolverFn};
use crate::program::{ProgramArgs, SolutionPart};
use std::fs;
use std::time::{Duration, Instant};

const SOLVERS: [[SolverFn; 2]; 7] = [
    [day1::solve_a, day1::solve_b],
    [day2::solve_a, day2::solve_b],
    [day3::solve_a, day3::solve_b],
    [day4::solve_a, day4::solve_b],
    [day5::solve_a, day5::solve_b],
    [day6::solve_a, day6::solve_b],
    [day7::solve_a, day7::solve_b],
];

fn get_solver(args: &ProgramArgs) -> AocResult<SolverFn> {
    if args.day() as usize > SOLVERS.len() {
        return Err(AocError::new("day not implemented"));
    }

    let part_index: usize = match args.part() {
        SolutionPart::A => 0,
        SolutionPart::B => 1,
    };
    return Ok(SOLVERS[(args.day() - 1) as usize][part_index]);
}

pub struct Solution {
    solution: iAoc,
    time: Duration,
}

impl Solution {
    pub fn new(solution: iAoc, time: Duration) -> Self {
        Solution { solution, time }
    }

    pub fn solution(&self) -> iAoc {
        self.solution
    }

    pub fn time(&self) -> &Duration {
        &self.time
    }
}

pub fn solve(args: &ProgramArgs) -> AocResult<Solution> {
    let solver = get_solver(args)?;
    let filename = match args.filename() {
        None => format!("input/{}.txt", args.day()),
        Some(filename) => format!("input/{}", filename),
    };
    let input = fs::read_to_string(filename).into_aoc_result()?;
    let now = Instant::now();
    let solution = solver(&input)?;
    let then = now.elapsed();
    Ok(Solution::new(solution, then))
}
