use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use std::str::FromStr;

type Range = (i32, i32);

fn intersects_range((a_left, a_right): &Range, (b_left, b_right): &Range) -> bool {
    !(a_right < b_left || a_left > b_right)
}

fn range_intersection((a_left, a_right): Range, (b_left, b_right): Range) -> Range {
    (a_left.max(b_left), a_right.min(b_right))
}

/// Original partitioning code.
/// Works great, but not the most optimal way to split cuboids.
#[allow(dead_code)]
fn partition_range(
    (a_left, a_right): Range,
    (b_left, b_right): Range,
) -> (Option<Range>, Option<Range>, Option<Range>) {
    let inner = if a_right < b_left || a_left > b_right {
        // A is completely to the left or right of B, so no overlap exists.
        None
    } else {
        // Get overlapping range by taking the rightmost left edge and leftmost right edge.
        Some((a_left.max(b_left), a_right.min(b_right)))
    };

    let outer_left = if a_right < b_left {
        // A is entirely to the left of B.
        Some((a_left, a_right))
    } else if a_left >= b_left {
        // Left edge of A is to the right of the left edge of B, no outer left range.
        None
    } else {
        // Left edge of A extends beyond left edge of B, an outer left range exists.
        Some((a_left, b_left - 1))
    };

    let outer_right = if a_left > b_right {
        // A is entirely to the right of B.
        Some((a_left, a_right))
    } else if a_right <= b_right {
        // Right edge of A is to the left of the right edge of B, no outer right range.
        None
    } else {
        // Right edge of A extends beyond right edge of B, an outer right range exists.
        Some((b_right + 1, a_right))
    };

    (outer_left, inner, outer_right)
}

#[derive(Debug)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    pub fn new(x: Range, y: Range, z: Range) -> Self {
        Cuboid { x, y, z }
    }
    pub fn cubes(&self) -> u64 {
        (self.x.1 - self.x.0 + 1) as u64
            * (self.y.1 - self.y.0 + 1) as u64
            * (self.z.1 - self.z.0 + 1) as u64
    }

    pub fn intersects(&self, other: &Cuboid) -> bool {
        intersects_range(&self.x, &other.x)
            && intersects_range(&self.y, &other.y)
            && intersects_range(&self.z, &other.z)
    }

    pub fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        if !self.intersects(other) {
            None
        } else {
            Some(Cuboid::new(
                range_intersection(self.x, other.x),
                range_intersection(self.y, other.y),
                range_intersection(self.z, other.z),
            ))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum CuboidState {
    Off,
    On,
}

#[derive(Debug)]
struct RebootStep {
    state: CuboidState,
    cuboid: Cuboid,
}

impl FromStr for RebootStep {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (state, ranges) = input.split_once(' ').into_aoc_result()?;
        let state = match state {
            "off" => CuboidState::Off,
            "on" => CuboidState::On,
            _ => return Err(AocError::new("invalid cuboid state")),
        };

        let mut ranges = ranges.split(',');

        let (x1, x2) = ranges.next().into_aoc_result()?[2..]
            .split_once("..")
            .into_aoc_result()?;
        let x = (
            x1.parse::<i32>().into_aoc_result()?,
            x2.parse::<i32>().into_aoc_result()?,
        );
        let (y1, y2) = ranges.next().into_aoc_result()?[2..]
            .split_once("..")
            .into_aoc_result()?;
        let y = (
            y1.parse::<i32>().into_aoc_result()?,
            y2.parse::<i32>().into_aoc_result()?,
        );
        let (z1, z2) = ranges.next().into_aoc_result()?[2..]
            .split_once("..")
            .into_aoc_result()?;
        let z = (
            z1.parse::<i32>().into_aoc_result()?,
            z2.parse::<i32>().into_aoc_result()?,
        );
        let cuboid = Cuboid::new(x, y, z);

        Ok(RebootStep { state, cuboid })
    }
}

fn parse_input(input: &str) -> AocResult<Vec<RebootStep>> {
    input
        .lines()
        .map(|line| RebootStep::from_str(line))
        .collect::<Result<_, _>>()
}

fn count_cubes(steps: Vec<RebootStep>) -> iAoc {
    let mut cuboids: Vec<Cuboid> = Vec::new();

    for RebootStep {
        state,
        cuboid: new_cuboid,
    } in steps
    {
        let mut new_cuboids = Vec::new();

        // For each existing cuboid, partition it into at most six new cuboids based
        // on the new cuboid being added.
        // This process removes overlapping ranges, replacing it with the new cuboid.
        for old_cuboid in cuboids {
            match old_cuboid.intersection(&new_cuboid) {
                // No intersection, old cuboid is unchanged.
                None => new_cuboids.push(old_cuboid),
                Some(intersection) => {
                    /*

                        An intersecting region exists, represented here.
                        The intersection will be a part of the new cuboid, so the
                        intersection must be subtracted from the old cuboid so it is not
                        counted twice in the volume.

                        There are multiple ways to do this. My initial solution was to
                        break up the outside regions of the old cuboid into a maximum
                        of 26 cuboids. This worst case happens when the new cuboid is
                        completely enclosed in an old cuboid. This partition method
                        grows much too quickly for the input.

                        A more efficient solution is to more cleverly group together
                        volumes of the old cuboid. The boundaries of the intersection
                        region are used to extend outside volumes to the intersection
                        region. Thus, "corners" and "edges" are not counted individually,
                        but they are clumped together as one piece.

                    */
                    if old_cuboid.x.0 < intersection.x.0 {
                        // X portion is left of new cuboid.
                        new_cuboids.push(Cuboid::new(
                            (old_cuboid.x.0, intersection.x.0 - 1),
                            old_cuboid.y,
                            old_cuboid.z,
                        ));
                    }
                    if old_cuboid.x.1 > intersection.x.1 {
                        // X portion is right of new cuboid.
                        new_cuboids.push(Cuboid::new(
                            (intersection.x.1 + 1, old_cuboid.x.1),
                            old_cuboid.y,
                            old_cuboid.z,
                        ));
                    }

                    // Notice that the X range of the old cuboid is no longer used for
                    // these regions, but the X range of the intersection region is.
                    // If an X range beyond the intersection region should be counted,
                    // it is assumed to have already been inserted in a different cuboid,
                    // which is asserted by the two if checks above this one, which check
                    // for external X portions and insert them as new cuboids as necessary.

                    if old_cuboid.y.0 < intersection.y.0 {
                        // Y portion is left of new cuboid.
                        new_cuboids.push(Cuboid::new(
                            intersection.x,
                            (old_cuboid.y.0, intersection.y.0 - 1),
                            old_cuboid.z,
                        ));
                    }
                    if old_cuboid.y.1 > intersection.y.1 {
                        // Y portion is right of new cuboid.
                        new_cuboids.push(Cuboid::new(
                            intersection.x,
                            (intersection.y.1 + 1, old_cuboid.y.1),
                            old_cuboid.z,
                        ));
                    }

                    // External Y region has already been inserted as a new cuboid,
                    // use Y range for intersection region for these next two cuboids.

                    if old_cuboid.z.0 < intersection.z.0 {
                        // Z portion is left of new cuboid.
                        new_cuboids.push(Cuboid::new(
                            intersection.x,
                            intersection.y,
                            (old_cuboid.z.0, intersection.z.0 - 1),
                        ));
                    }
                    if old_cuboid.z.1 > intersection.z.1 {
                        // Z portion is right of new cuboid.
                        new_cuboids.push(Cuboid::new(
                            intersection.x,
                            intersection.y,
                            (intersection.z.1 + 1, old_cuboid.z.1),
                        ));
                    }
                }
            }
        }
        if state == CuboidState::On {
            new_cuboids.push(new_cuboid);
        }

        cuboids = new_cuboids;
    }

    cuboids
        .into_iter()
        .fold(0 as iAoc, |acc, cuboid| acc + cuboid.cubes())
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let steps = parse_input(input)?;
    let init_area = Cuboid::new((-50, 50), (-50, 50), (-50, 50));
    let steps = steps
        .into_iter()
        .filter(|RebootStep { cuboid, .. }| {
            return cuboid.intersects(&init_area);
        })
        .collect::<Vec<_>>();
    let result = count_cubes(steps);
    Ok(result)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let steps = parse_input(input)?;
    let result = count_cubes(steps);
    Ok(result)
}
