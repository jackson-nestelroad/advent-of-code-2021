use crate::common::AocResult;

#[allow(non_camel_case_types)]
pub type iAoc = u64;

pub type SolverFn = fn(&str) -> AocResult<iAoc>;
