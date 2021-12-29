use super::*;
use crate::common::{iAoc, AocError, AocResult, IntoAocResult, SolverFn};
use crate::program::{ProgramArgs, SolutionPart};
use std::fs;
use std::time::{Duration, Instant};

const SOLVERS: [[SolverFn; 2]; 25] = [
    [day01::solve_a, day01::solve_b],
    [day02::solve_a, day02::solve_b],
    [day03::solve_a, day03::solve_b],
    [day04::solve_a, day04::solve_b],
    [day05::solve_a, day05::solve_b],
    [day06::solve_a, day06::solve_b],
    [day07::solve_a, day07::solve_b],
    [day08::solve_a, day08::solve_b],
    [day09::solve_a, day09::solve_b],
    [day10::solve_a, day10::solve_b],
    [day11::solve_a, day11::solve_b],
    [day12::solve_a, day12::solve_b],
    [day13::solve_a, day13::solve_b],
    [day14::solve_a, day14::solve_b],
    [day15::solve_a, day15::solve_b],
    [day16::solve_a, day16::solve_b],
    [day17::solve_a, day17::solve_b],
    [day18::solve_a, day18::solve_b],
    [day19::solve_a, day19::solve_b],
    [day20::solve_a, day20::solve_b],
    [day21::solve_a, day21::solve_b],
    [day22::solve_a, day22::solve_b],
    [day23::solve_a, day23::solve_b],
    [day24::solve_a, day24::solve_b],
    [day25::solve_a, day25::solve_b],
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
