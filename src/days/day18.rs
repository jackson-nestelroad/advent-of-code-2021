use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use itertools::Itertools;
use num::Integer;
use std::str::FromStr;

/// Stores each node value and its depth rather than the entire tree structure.
/// Makes finding neighbors extremely easy, but tree operations are a bit more
/// difficult to implement.
#[derive(Clone)]
struct SnailfishNumber {
    values: Vec<u64>,
    depths: Vec<u8>,
}

impl SnailfishNumber {
    pub fn add(&self, other: &SnailfishNumber) -> Self {
        let mut sum = self.clone();
        sum.values.extend(other.values.iter());
        sum.depths.extend(other.depths.iter());
        for depth in sum.depths.iter_mut() {
            *depth += 1;
        }
        sum
    }

    fn is_pair(&self, i: usize) -> bool {
        // This condition holds if scanning is performed one end of the vector
        // to the other. Starting a scan at any position except the beginning
        // or end causes this condition to fail, because the actual position
        // of each node relative to its parent is not stored.
        self.depths
            .get(i + 1)
            .and_then(|right| Some(*right == self.depths[i]))
            .unwrap_or(false)
    }

    pub fn reduce(&mut self) {
        while self.reduce_once() {}
    }

    pub fn reduce_once(&mut self) -> bool {
        for i in 0..self.values.len() {
            if self.is_pair(i) && self.depths[i] == 4 {
                self.explode(i);
                return true;
            }
        }
        for i in 0..self.values.len() {
            if self.values[i] >= 10 {
                self.split(i);
                return true;
            }
        }
        false
    }

    fn explode(&mut self, i: usize) {
        if i != 0 {
            self.values[i - 1] += self.values[i];
        }

        let right_neighbor = i + 2;
        if right_neighbor < self.values.len() {
            self.values[right_neighbor] += self.values[i + 1];
        }

        self.values[i] = 0;
        self.depths[i] -= 1;

        self.values.remove(i + 1);
        self.depths.remove(i + 1);
    }

    fn split(&mut self, i: usize) {
        let value = self.values[i];
        let left = value.div_floor(&2);
        let right = value - left;

        self.values[i] = left;
        self.depths[i] += 1;

        self.values.insert(i + 1, right);
        self.depths.insert(i + 1, self.depths[i]);
    }

    pub fn magnitude(mut self) -> u64 {
        // Reduce the first pair from left to right until there is only one
        // value remaining.
        while self.values.len() > 1 {
            for i in 0..self.values.len() {
                if self.is_pair(i) {
                    self.values[i] = 3 * self.values[i] + 2 * self.values[i + 1];
                    if self.depths[i] > 0 {
                        self.depths[i] -= 1;
                    }

                    self.values.remove(i + 1);
                    self.depths.remove(i + 1);

                    break;
                }
            }
        }
        self.values[0]
    }
}

impl FromStr for SnailfishNumber {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = SnailfishNumber {
            values: Vec::new(),
            depths: Vec::new(),
        };
        let mut depth = 0;
        for ch in input.trim().chars() {
            match ch {
                '[' => depth += 1,
                ']' => {
                    if depth == 0 {
                        return Err(AocError::new("malformed snailfish number"));
                    }
                    depth -= 1;
                }
                ',' => (),
                digit => {
                    result
                        .values
                        .push(digit.to_digit(10).into_aoc_result()? as u64);
                    result.depths.push(depth - 1);
                }
            }
        }
        Ok(result)
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let numbers: Vec<SnailfishNumber> = input
        .lines()
        .map(|line| SnailfishNumber::from_str(line))
        .collect::<Result<_, _>>()?;

    let mut numbers_iter = numbers.into_iter();
    let mut sum = numbers_iter.next().into_aoc_result()?;
    numbers_iter.fold((), |_, b| {
        sum = sum.add(&b);
        sum.reduce();
    });
    Ok(sum.magnitude())
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let numbers: Vec<SnailfishNumber> = input
        .lines()
        .map(|line| SnailfishNumber::from_str(line))
        .collect::<Result<_, _>>()?;

    let result = numbers
        .iter()
        .enumerate()
        .cartesian_product(numbers.iter().enumerate())
        .filter_map(|((i, a), (j, b))| {
            if i == j {
                None
            } else {
                let mut sum = a.add(b);
                sum.reduce();
                Some(sum.magnitude())
            }
        })
        .max()
        .into_aoc_result()?;
    Ok(result)
}
