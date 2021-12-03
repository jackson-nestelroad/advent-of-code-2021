use crate::common::Error;

#[allow(non_camel_case_types)]
pub type iAoC = u64;

pub type SolverFn = fn(&str) -> Result<iAoC, Error>;
