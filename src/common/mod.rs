mod error;
pub mod point;
mod solver;

pub use error::{AocError, AocResult, IntoAocResult};
pub use solver::{iAoc, SolverFn};
