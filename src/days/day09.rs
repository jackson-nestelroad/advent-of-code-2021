use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

struct HeightMap {
    map: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl HeightMap {
    pub fn new(map: Vec<Vec<u32>>) -> Self {
        let height = map.len();
        let width = if let Some(row) = map.first() {
            row.len()
        } else {
            0
        };
        HeightMap { map, height, width }
    }

    pub fn get(&self, (row, col): (usize, usize)) -> u32 {
        if row >= self.height || col >= self.width {
            9
        } else {
            self.map[row][col]
        }
    }

    pub fn is_low_point(&self, point: (usize, usize)) -> bool {
        let pos = self.get(point);
        self.get_neighbors(point).iter().all(|neighbor| {
            pos < if let Some(neighbor) = neighbor {
                self.get(*neighbor)
            } else {
                9
            }
        })
    }

    pub fn get_neighbors(&self, (row, col): (usize, usize)) -> [Option<(usize, usize)>; 4] {
        [
            if row == 0 { None } else { Some((row - 1, col)) },
            Some((row + 1, col)),
            if col == 0 { None } else { Some((row, col - 1)) },
            Some((row, col + 1)),
        ]
    }
}

impl FromStr for HeightMap {
    type Err = AocError;

    fn from_str(input: &str) -> AocResult<Self> {
        Ok(HeightMap::new(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| ch.to_digit(10).into_aoc_result())
                        .collect::<AocResult<_>>()
                })
                .collect::<AocResult<_>>()?,
        ))
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let height_map = HeightMap::from_str(input)?;
    let mut sum_risk_levels = 0;
    for row in 0..height_map.height {
        for col in 0..height_map.width {
            let point = (row, col);
            if height_map.is_low_point(point) {
                sum_risk_levels += height_map.get(point) + 1;
            }
        }
    }
    Ok(sum_risk_levels as iAoc)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let height_map = HeightMap::from_str(input)?;

    let mut basin_sizes: Vec<usize> = Vec::new();
    for row in 0..height_map.height {
        for col in 0..height_map.width {
            let point = (row, col);
            if height_map.is_low_point(point) {
                // Found a low point, now build the basin by exploring around it.

                // Points in the basin.
                let mut basin = HashSet::new();
                // Points to explore.
                let mut explore_queue = VecDeque::new();
                explore_queue.push_back(point);

                while !explore_queue.is_empty() {
                    let point = explore_queue.pop_front().unwrap();
                    basin.insert(point);
                    for neighbor in height_map.get_neighbors(point) {
                        if let Some(neighbor) = neighbor {
                            if !basin.contains(&neighbor) && height_map.get(neighbor) != 9 {
                                explore_queue.push_back(neighbor);
                            }
                        }
                    }
                }
                basin_sizes.push(basin.len());
            }
        }
    }

    if basin_sizes.len() < 3 {
        return Err(AocError::new("did not find 3 basins"));
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    let result = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

    Ok(result as iAoc)
}
