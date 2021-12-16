use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use num::Integer;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

type Point = (usize, usize);

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn manhatten_distance((x1, y1): &Point, (x2, y2): &Point) -> usize {
    let dist_x = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let dist_y = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    dist_x + dist_y
}

struct Cavern {
    flat_grid: Vec<u32>,
    height: usize,
    width: usize,
}

impl FromStr for Cavern {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let height = input.lines().count();
        let width = input.lines().next().into_aoc_result_msg("no rows")?.len();
        let flat_grid = input
            .lines()
            .flat_map(|line| line.chars().map(|ch| ch.to_digit(10).into_aoc_result()))
            .collect::<Result<_, _>>()?;
        Ok(Cavern {
            flat_grid,
            height,
            width,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathState {
    position: Point,
    cost: usize,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cavern {
    pub fn get(&self, (x, y): &Point) -> Option<u32> {
        let (cluster_y, base_y) = y.div_mod_floor(&self.height);
        let (cluster_x, base_x) = x.div_mod_floor(&self.width);
        if cluster_y > 5 || cluster_x > 5 {
            None
        } else {
            match self.width.overflowing_mul(base_y) {
                (_, true) => None,
                (offset, false) => match offset.overflowing_add(base_x) {
                    (_, true) => None,
                    (index, false) => self.flat_grid.get(index).copied().and_then(|value| {
                        Some((value as usize + cluster_y + cluster_x - 1).mod_floor(&9) as u32 + 1)
                    }),
                },
            }
        }
    }

    /// Finds the safest path using the A* algorithm.
    pub fn safest_path(&self, start: Point, end: Point) -> AocResult<usize> {
        // Heuristic function uses the distance between the current point and end point.
        let h = |point: &Point| manhatten_distance(point, &end);

        let start_f_score = h(&start);

        let mut f_scores = HashMap::new();
        f_scores.insert(start, start_f_score);

        let mut g_scores = HashMap::new();
        g_scores.insert(start, 0);

        let mut open_set = BinaryHeap::new();
        open_set.push(PathState {
            position: start,
            cost: start_f_score,
        });

        while let Some(PathState {
            position,
            cost: f_score,
        }) = open_set.pop()
        {
            // We have reached our destination.
            if position == end {
                return Ok(f_score);
            }

            // We have found a better path than this one, so ignore it.
            if f_score > f_scores.get(&position).copied().unwrap_or(usize::MAX) {
                continue;
            }

            let g_score = g_scores.get(&position).copied().unwrap();

            for (dx, dy) in NEIGHBORS {
                let neighbor = (
                    position.0.overflowing_add(dx as usize).0,
                    position.1.overflowing_add(dy as usize).0,
                );
                if let Some(neighbor_cost) = self.get(&neighbor) {
                    let tentative_g_score = g_score + neighbor_cost;
                    let neighbor_g_score = g_scores.entry(neighbor).or_insert(u32::MAX);
                    if tentative_g_score < *neighbor_g_score {
                        let new_f_score = tentative_g_score as usize + h(&neighbor);
                        *f_scores.entry(neighbor).or_default() = new_f_score;
                        *neighbor_g_score = tentative_g_score;
                        open_set.push(PathState {
                            position: neighbor,
                            cost: new_f_score,
                        });
                    }
                }
            }
        }

        Err(AocError::new("no path found"))
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let cavern = Cavern::from_str(input)?;
    let result = cavern.safest_path((0, 0), (cavern.width - 1, cavern.height - 1))?;
    Ok(result as iAoc)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let cavern = Cavern::from_str(input)?;
    let result = cavern.safest_path((0, 0), (5 * cavern.width - 1, 5 * cavern.height - 1))?;
    Ok(result as iAoc)
}
