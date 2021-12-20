use crate::common::{iAoc, AocResult, IntoAocResult};
use itertools::Itertools;
use lazy_static::lazy_static;
use num::{Integer, Unsigned};
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::{Add, Index, Mul, MulAssign, Sub};

/// A single point, which can represent a beacon or scanner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point((i32, i32, i32));

impl Point {
    /// Returns the manhatten distance between two points.
    pub fn distance(&self, other: &Point) -> usize {
        ((self.0 .0 - other.0 .0).abs()
            + (self.0 .1 - other.0 .1).abs()
            + (self.0 .2 - other.0 .2).abs()) as usize
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point((
            self.0 .0 + rhs.0 .0,
            self.0 .1 + rhs.0 .1,
            self.0 .2 + rhs.0 .2,
        ))
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point((
            self.0 .0 - rhs.0 .0,
            self.0 .1 - rhs.0 .1,
            self.0 .2 - rhs.0 .2,
        ))
    }
}

/// A single axis in a 3D plane.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Axis {
    X,
    Y,
    Z,
}

/// A positive or negative sign, which represents a direction along an axis.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Sign {
    Positive,
    Negative,
}

impl Mul<Sign> for Sign {
    type Output = Sign;

    fn mul(self, rhs: Sign) -> Self::Output {
        if self == rhs {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

/// A single row in a transformation matrix, which represents a single
/// axis, which represents a single row in the identity matrix, and a
/// sign, which represents a potential reflection.
///
/// X => (1,0,0)
/// Y => (0,1,0)
/// Z => (0,0,1)
#[derive(Copy, Clone, Debug)]
struct Transformation(Axis, Sign);

impl Mul<Sign> for &Transformation {
    type Output = Transformation;

    fn mul(self, rhs: Sign) -> Self::Output {
        Transformation(self.0, self.1 * rhs)
    }
}

impl MulAssign<Sign> for Transformation {
    fn mul_assign(&mut self, rhs: Sign) {
        self.1 = self.1 * rhs
    }
}

impl Mul<&Point> for &Transformation {
    type Output = i32;

    fn mul(self, rhs: &Point) -> Self::Output {
        match (self.0, self.1) {
            (Axis::X, Sign::Positive) => rhs.0 .0,
            (Axis::X, Sign::Negative) => -rhs.0 .0,
            (Axis::Y, Sign::Positive) => rhs.0 .1,
            (Axis::Y, Sign::Negative) => -rhs.0 .1,
            (Axis::Z, Sign::Positive) => rhs.0 .2,
            (Axis::Z, Sign::Negative) => -rhs.0 .2,
        }
    }
}

/// A transformation matrix, simplified down to three transformations.
#[derive(Debug)]
struct TransformationMatrix([Transformation; 3]);

impl Index<usize> for TransformationMatrix {
    type Output = Transformation;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl Mul<&Point> for &TransformationMatrix {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point((&self.0[0] * rhs, &self.0[1] * rhs, &self.0[2] * rhs))
    }
}

/// A single scanner and its collection of known beacons.
struct Scanner {
    beacons: FxHashSet<Point>,
}

/// A scanner, its collection of known beacons, and a set of the distances
/// between those beacons.
struct ScannerWithDistancesToBeacons {
    beacons: FxHashSet<Point>,
    // Maps a distance to a vector of beacons that have another beacon that
    // distance away from it.
    distances: FxHashMap<usize, Vec<Point>>,
}

/// A global map of known scanners and their corresponding beacon data.
/// Scanner data is translated and oriented properly before inserted into the global map.
struct GlobalMap {
    scanners: FxHashMap<Point, ScannerWithDistancesToBeacons>,
}

impl GlobalMap {
    pub fn new() -> Self {
        Self {
            scanners: FxHashMap::default(),
        }
    }

    pub fn from_scanners(scanners: Vec<Scanner>) -> Self {
        let mut scanners = scanners
            .into_iter()
            .map(|scan| scan.into_distances())
            .collect::<Vec<_>>();

        let mut global_map = GlobalMap::new();

        // Use the first scanner as the origin. Everything will be relative to
        // the first scanner's orientation.
        global_map
            .scanners
            .insert(Point((0, 0, 0)), scanners.remove(0));

        while !scanners.is_empty() {
            for i in (0..scanners.len()).rev() {
                if global_map.merge_scanner(&scanners[i]) {
                    scanners.swap_remove(i);
                }
            }
        }
        global_map
    }

    pub fn merge_scanner(&mut self, scanner: &ScannerWithDistancesToBeacons) -> bool {
        // 12 overlaps are needed between beacons in two beacon sets to be valid for merging.
        const DESIRED_OVERLAPS: usize = 12;
        lazy_static! {
            // To detect if 12 beacons will overlap with the global map, C(12,2) lines between
            // all of those beacons must have identical length with distances in the global map.
            static ref DISTANCE_OVERLAPS: usize = combinations(DESIRED_OVERLAPS, 2);
        }

        // Set of distances in the current scanner.
        let scanned_distances = scanner.distances.keys().copied().collect::<FxHashSet<_>>();

        // Find one known scanner that this scanner can be merged with.
        for (_, known_scanner) in &self.scanners {
            // Distances we know and have properly oriented for this known scanner.
            let known_distances = known_scanner
                .distances
                .keys()
                .copied()
                .collect::<FxHashSet<_>>();

            // Distances that overlap between the two scanners.
            let overlapping_distances = known_distances
                .intersection(&scanned_distances)
                .copied()
                .collect::<FxHashSet<_>>();

            if overlapping_distances.len() >= *DISTANCE_OVERLAPS {
                // This scanner has 12 beacons that can be mapped to known beacons in the global map.
                // We now must find how to properly orient and translate these beacons to actually
                // match the 12 beacons in the global map.
                for transformation_matrix in BeaconOrientationIterator::new() {
                    // Start by creating a transformed distance map for the new scanner.
                    // This map maps an overlapping distance (from the overlapping_distances set)
                    // to a vector of transformed beacons that have another beacon that distance
                    // away from it.
                    let overlapping_distance_to_transformed_beacons = overlapping_distances
                        .iter()
                        .map(|dist| {
                            (
                                *dist,
                                scanner.distances[dist]
                                    .iter()
                                    .map(|beacon| &transformation_matrix * &beacon)
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<FxHashMap<_, _>>();
                    // We derive the potential translations by pairing up all points with the same
                    // distance from another beacon with each other and taking the difference.
                    // This difference is the translation between the two points, which also represents
                    // the location of the new scanner relative to the origin.
                    //
                    // One of these translations will work, and we check by translating the entire
                    // new beacon set and checking if 12 points match up.
                    let potential_translations = overlapping_distances
                        .iter()
                        .flat_map(|dist| {
                            known_scanner.distances[dist].iter().cartesian_product(
                                overlapping_distance_to_transformed_beacons[dist].iter(),
                            )
                        })
                        .map(|(known_beacon, unknown_beacon)| known_beacon - &unknown_beacon)
                        .collect::<Vec<_>>();

                    for delta in potential_translations {
                        // Go ahead and perform all of the transformations now.
                        // You really only need to check for points that correspond to overlapping
                        // distances, but each scanner does not have that many points, so it does
                        // not cost much to go ahead and translate them all.
                        let all_oriented_beacons = scanner
                            .beacons
                            .iter()
                            .map(|beacon| &(&transformation_matrix * &beacon) + &delta)
                            .collect::<FxHashSet<_>>();

                        if all_oriented_beacons
                            .iter()
                            .filter(|beacon| known_scanner.beacons.contains(beacon))
                            .count()
                            >= DESIRED_OVERLAPS
                        {
                            // Insert the scanner's beacons with the proper orientation.
                            let scanner = Scanner {
                                beacons: all_oriented_beacons,
                            }
                            .into_distances();
                            self.scanners.insert(delta, scanner);
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn beacons(&self) -> FxHashSet<&Point> {
        self.scanners
            .values()
            .flat_map(|scanner| scanner.beacons.iter())
            .collect()
    }

    pub fn scanners(&self) -> FxHashSet<&Point> {
        self.scanners.keys().collect()
    }
}

impl Scanner {
    pub fn into_distances(self) -> ScannerWithDistancesToBeacons {
        let pairs = self.beacons.iter().tuple_combinations();
        let mut distances = FxHashMap::default();
        for (a, b) in pairs {
            let entry = distances.entry(a.distance(b)).or_insert(Vec::new());
            entry.push(*a);
            entry.push(*b);
        }
        ScannerWithDistancesToBeacons {
            beacons: self.beacons,
            distances,
        }
    }
}

fn parse_input(input: &str) -> AocResult<Vec<Scanner>> {
    let mut scans = Vec::new();
    for line in input.lines() {
        if line.starts_with("---") {
            scans.push(Scanner {
                beacons: FxHashSet::default(),
            });
        } else if !line.is_empty() {
            let mut nums = line
                .split(',')
                .map(|num| num.parse::<i32>().into_aoc_result());
            scans.last_mut().into_aoc_result()?.beacons.insert(Point((
                nums.next().into_aoc_result()??,
                nums.next().into_aoc_result()??,
                nums.next().into_aoc_result()??,
            )));
        }
    }
    Ok(scans)
}

/// Iterator for iterating through all possible orientation transformations.
struct BeaconOrientationIterator {
    /// First row of the transformation matrix.
    i: usize,
    /// Second row of the transformation matrix.
    j: usize,
    /// How to negate the first two rows of the transformation matrix.
    k: usize,
    // The third row of the transformation matrix is the cross product
    // of the first two rows.
}

impl BeaconOrientationIterator {
    const IDENTITY: TransformationMatrix = TransformationMatrix([
        Transformation(Axis::X, Sign::Positive),
        Transformation(Axis::Y, Sign::Positive),
        Transformation(Axis::Z, Sign::Positive),
    ]);

    pub fn new() -> Self {
        BeaconOrientationIterator { i: 0, j: 1, k: 0 }
    }

    fn done(&self) -> bool {
        self.i == 3
    }

    fn advance_index(&mut self) {
        self.j += 1;
        if self.j == self.i {
            self.advance_index();
        } else if self.j >= 3 {
            self.i += 1;
            self.j = 0;
        }
    }

    fn advance_state(&mut self) {
        self.k += 1;
        if self.k >= 4 {
            self.k = 0;
            self.advance_index();
        }
    }
}

impl Iterator for BeaconOrientationIterator {
    type Item = TransformationMatrix;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done() {
            return None;
        }

        let mut first = Self::IDENTITY[self.i];
        let mut second = Self::IDENTITY[self.j];

        let mut third = {
            let next_i = (self.i + 1).mod_floor(&3);
            let next_j = (self.j + 1).mod_floor(&3);
            if next_i == self.j {
                Self::IDENTITY[next_j]
            } else {
                &Self::IDENTITY[next_i] * Sign::Negative
            }
        };

        match self.k {
            0 => (),
            1 => {
                second *= Sign::Negative;
                third *= Sign::Negative;
            }
            2 => {
                first *= Sign::Negative;
                second *= Sign::Negative;
            }
            3 => {
                first *= Sign::Negative;
                third *= Sign::Negative;
            }
            _ => unreachable!(),
        }

        let result = TransformationMatrix([first, second, third]);
        self.advance_state();
        Some(result)
    }
}

fn factorial<I: Integer + Unsigned + Clone + num::ToPrimitive + std::iter::Product>(n: I) -> I {
    num::range_inclusive(I::one(), n).product()
}

fn combinations<I: Integer + Unsigned + Clone + Copy + num::ToPrimitive + std::iter::Product>(
    n: I,
    r: I,
) -> I {
    num::range_inclusive(n - r + I::one(), n)
        .product::<I>()
        .div_floor(&factorial(r))
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let scanners = parse_input(input)?;
    let global_map = GlobalMap::from_scanners(scanners);
    Ok(global_map.beacons().len() as iAoc)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let scanners = parse_input(input)?;
    let global_map = GlobalMap::from_scanners(scanners);

    let result = global_map
        .scanners()
        .iter()
        .tuple_combinations()
        .map(|(from, to)| from.distance(to))
        .max()
        .into_aoc_result()?;
    Ok(result as iAoc)
}
