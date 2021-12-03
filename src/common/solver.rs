use crate::common::Error;

#[allow(non_camel_case_types)]
pub type iAoC = i64;

pub type SolverFn = fn(&str) -> Result<iAoC, Error>;

pub trait Solver {
    fn solve_a(input: &str) -> Result<iAoC, Error>;
    fn solve_b(input: &str) -> Result<iAoC, Error>;
}
