use crate::common::point::Point;
use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use num::range_step_inclusive;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

struct LineSegment {
    pub begin: Point<i32>,
    pub end: Point<i32>,
}

impl FromStr for LineSegment {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (first, second) = input.split_once(" -> ").into_aoc_result()?;
        let (x1, y1) = first.split_once(',').into_aoc_result()?;
        let (x2, y2) = second.split_once(',').into_aoc_result()?;
        Ok(LineSegment {
            begin: Point::new(
                x1.parse::<i32>().into_aoc_result()?,
                y1.parse::<i32>().into_aoc_result()?,
            ),
            end: Point::new(
                x2.parse::<i32>().into_aoc_result()?,
                y2.parse::<i32>().into_aoc_result()?,
            ),
        })
    }
}

fn create_grid(segments: Vec<LineSegment>) -> HashMap<Point<i32>, i32> {
    let mut grid = HashMap::new();
    for seg in segments {
        // Do not need to worry about slope due to guarantee of the problem,
        // which states all lines are horizontal, vertical, or 45-degree diagonal.
        let dy = seg.end.y.cmp(&seg.begin.y);
        let dx = seg.end.x.cmp(&seg.begin.x);

        if dx == Ordering::Equal {
            if dy != Ordering::Equal {
                for y in range_step_inclusive(seg.begin.y, seg.end.y, dy as i32) {
                    *grid.entry(Point::new(seg.begin.x, y)).or_insert(0) += 1;
                }
            }
        } else if dy == Ordering::Equal {
            for x in range_step_inclusive(seg.begin.x, seg.end.x, dx as i32) {
                *grid.entry(Point::new(x, seg.begin.y)).or_insert(0) += 1;
            }
        } else {
            for (x, y) in range_step_inclusive(seg.begin.x, seg.end.x, dx as i32)
                .zip(range_step_inclusive(seg.begin.y, seg.end.y, dy as i32))
            {
                *grid.entry(Point::new(x, y)).or_insert(0) += 1;
            }
        }
    }
    grid
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let mut segments: Vec<LineSegment> = input
        .lines()
        .map(|line| LineSegment::from_str(line))
        .collect::<Result<_, _>>()
        .into_aoc_result()?;
    segments = segments
        .into_iter()
        .filter(|seg| seg.begin.x == seg.end.x || seg.begin.y == seg.end.y)
        .collect();

    let grid = create_grid(segments);
    let result = grid.values().filter(|&&overlaps| overlaps >= 2).count();

    Ok(result as iAoc)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let segments: Vec<LineSegment> = input
        .lines()
        .map(|line| LineSegment::from_str(line))
        .collect::<Result<_, _>>()
        .into_aoc_result()?;

    let grid = create_grid(segments);
    let result = grid.values().filter(|&&overlaps| overlaps >= 2).count();

    Ok(result as iAoc)
}
