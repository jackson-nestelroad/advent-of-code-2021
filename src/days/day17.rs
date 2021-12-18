use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use itertools::Itertools;
use num::integer::Roots;
use num::Integer;
use std::cmp::Ordering;
use std::str::FromStr;

type Point = (i32, i32);

struct TargetArea {
    min: Point,
    max: Point,
}

impl TargetArea {
    pub fn in_area(&self, point: &Point) -> bool {
        point.0 >= self.min.0
            && point.1 >= self.min.1
            && point.0 <= self.max.0
            && point.1 <= self.max.1
    }
}

impl FromStr for TargetArea {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (xs, ys) = input
            .trim()
            .split_once("target area: ")
            .into_aoc_result()?
            .1
            .split_once(", ")
            .into_aoc_result()?;
        let mut xs = xs.split(&['=', '.'][..]).skip(1);
        let mut ys = ys.split(&['=', '.'][..]).skip(1);
        let min = (
            xs.next()
                .into_aoc_result()?
                .parse::<i32>()
                .into_aoc_result()?,
            ys.next()
                .into_aoc_result()?
                .parse::<i32>()
                .into_aoc_result()?,
        );
        xs.next();
        ys.next();
        let max = (
            xs.next()
                .into_aoc_result()?
                .parse::<i32>()
                .into_aoc_result()?,
            ys.next()
                .into_aoc_result()?
                .parse::<i32>()
                .into_aoc_result()?,
        );
        Ok(TargetArea { min, max })
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let target = TargetArea::from_str(input)?;

    /*
        To get the largest maximum height, we want the largest initial Y velocity that
        still hits the target area.
        Let v_y be the initial Y velocity.
        Due to gravity, the Y position will eventually return to 0 (the starting point)
        at step t = 2 * v_y + 1.

        At step t + 1, the Y position will continue decreasing, and the next position
        will be y = -(v_y + 1) = -v_y - 1

        For y to be the in the target area, min_y <= -v_y - 1 <= max_y.

        This inequality can be easily solved to make v_y as large as possible by only
        considering the minimum y value in the target area.

            -v_y - 1 = min_y

            v_y = -min_y - 1

        Then, the highest value reached will be \sum_{i=0}{v_y} i, which is the sum of
        all integers from 0 to v_y, which equals (v_y + 1)(v_y)/2.
    */

    let min_y = target.min.1;
    let v_y = -min_y - 1;
    let peak = ((v_y + 1) * v_y).div_floor(&2);
    Ok(peak as iAoc)
}

struct TrajectoryIterator {
    v_x: i32,
    v_y: i32,
    pos: Point,
}

impl TrajectoryIterator {
    pub fn new(pos: Point, v_x: i32, v_y: i32) -> Self {
        TrajectoryIterator { v_x, v_y, pos }
    }
}

impl Iterator for TrajectoryIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos = (self.pos.0 + self.v_x, self.pos.1 + self.v_y);
        match self.v_x.cmp(&0) {
            Ordering::Less => self.v_x += 1,
            Ordering::Greater => self.v_x -= 1,
            Ordering::Equal => (),
        }
        self.v_y -= 1;
        Some(self.pos)
    }
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let target = TargetArea::from_str(input)?;
    // The minimum initial Y velocity goes directly to the bottom of the target area
    // in the first step.
    let min_v_y = target.min.1;

    // The maximum initial Y velocity was described in part A.
    let max_v_y = -target.min.1 - 1;

    /*
        The minimum initial X velocity causes the X velocity to become 0 after reaching the
        leftmost edge of the target area. If we never reach the target area, the velocity
        can trivially not be counted.

        The final X position reached by an initial X velocity v_x is \sum_{i=0}{v_x} i,
        which is the sum of all integers from 0 to v_x, which equals (v_x + 1)(v_x)/2.

        Thus, to be a valid X velocity, the following inequality must hold:

            min_x <= (v_x + 1)(v_x) / 2 <= max_x

        It should be noted, however, that there is actually no maximum valid initial X
        velocity, because the Y velocity may enter the target area before the X velocity
        reaches 0. The actual maximum initial X velocity would be max_x, which hits the
        rightmost edge of the target area in a single step.

        To minimize v_x, we only consider the v_x in relation to min_x:

            (v_x + 1)(v_x) / 2 = min_x
            (v_x + 1)(v_x) = 2 * min_x
            v_x^2 + v_x - 2 * min_x = 0

        The above equation can be solved using the quadratic formula, which can be
        simplified to:

            min_v_x = (-1 + sqrt(8 * min_x + 1)) / 2
    */
    let min_v_x = (-1 + (8 * target.min.0 + 1).sqrt()).div_ceil(&2);
    let max_v_x = target.max.0;

    // Now count all valid velocity pairs.
    let result = (min_v_x..=max_v_x)
        .cartesian_product(min_v_y..=max_v_y)
        .map(|(v_x, v_y)| TrajectoryIterator::new((0, 0), v_x, v_y))
        .filter_map(|trajectory| {
            for pos in trajectory {
                if pos.0 > target.max.0 || pos.1 < target.min.1 {
                    // Passed the boundaries of the target area in a way that
                    // the target area will never be reached.
                    return None;
                } else if target.in_area(&pos) {
                    return Some(());
                }
            }
            unreachable!();
        })
        .count();
    Ok(result as iAoc)
}
