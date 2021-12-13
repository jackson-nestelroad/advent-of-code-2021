use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

type Point = (usize, usize);

#[derive(Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

enum PaperInstructionsParsingState {
    Points,
    Folds,
}

struct PaperInstructions {
    points: HashSet<Point>,
    fold_lines: Vec<Fold>,
}

impl FromStr for PaperInstructions {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = PaperInstructions::new();

        let mut state = PaperInstructionsParsingState::Points;

        for line in input.lines() {
            if line.is_empty() {
                state = PaperInstructionsParsingState::Folds;
            } else {
                match state {
                    PaperInstructionsParsingState::Points => {
                        let (x, y) = line.split_once(',').into_aoc_result()?;
                        result.points.insert((
                            x.parse::<usize>().into_aoc_result()?,
                            y.parse::<usize>().into_aoc_result()?,
                        ));
                    }
                    PaperInstructionsParsingState::Folds => {
                        let equals_index = line.find('=').into_aoc_result()?;
                        if equals_index == 0 {
                            return Err(AocError::new("invalid fold line"));
                        }
                        let num = line
                            .get((equals_index + 1)..)
                            .into_aoc_result()?
                            .parse::<usize>()
                            .into_aoc_result()?;
                        let line_ascii = line.as_bytes();
                        result
                            .fold_lines
                            .push(match line_ascii[equals_index - 1] as char {
                                'x' => Fold::X(num),
                                'y' => Fold::Y(num),
                                _ => return Err(AocError::new("invalid fold line")),
                            });
                    }
                }
            }
        }

        Ok(result)
    }
}

impl PaperInstructions {
    pub fn new() -> Self {
        PaperInstructions {
            points: HashSet::new(),
            fold_lines: Vec::new(),
        }
    }

    fn fold(points: HashSet<Point>, fold: Fold) -> HashSet<Point> {
        match fold {
            Fold::X(fold_x) => {
                let (mut left, right): (HashSet<Point>, HashSet<Point>) =
                    points.iter().partition(|(x, _)| *x < fold_x);
                for (x, y) in right {
                    left.insert((fold_x - (x - fold_x), y));
                }
                left
            }
            Fold::Y(fold_y) => {
                let (mut top, bottom): (HashSet<Point>, HashSet<Point>) =
                    points.iter().partition(|(_, y)| *y < fold_y);
                for (x, y) in bottom {
                    top.insert((x, fold_y - (y - fold_y)));
                }
                top
            }
        }
    }

    pub fn into_folded(self) -> Self {
        let mut points = self.points;
        for fold in self.fold_lines {
            points = PaperInstructions::fold(points, fold);
        }
        PaperInstructions {
            points,
            fold_lines: Vec::new(),
        }
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let instr = PaperInstructions::from_str(input)?;
    let result = PaperInstructions::fold(
        instr.points,
        *instr
            .fold_lines
            .first()
            .into_aoc_result_msg("no first fold")?,
    )
    .len();
    Ok(result as iAoc)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let instr = PaperInstructions::from_str(input)?;
    let folded = instr.into_folded();

    let max_x = folded
        .points
        .iter()
        .map(|(x, _)| x)
        .max()
        .into_aoc_result()?;
    let max_y = folded
        .points
        .iter()
        .map(|(_, y)| y)
        .max()
        .into_aoc_result()?;

    let mut grid_raw = vec![' ' as u8; (max_x + 2) * (max_y + 1)];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(max_x + 2).collect();
    let grid = grid_base.as_mut_slice();

    for (x, y) in &folded.points {
        grid[*y][*x] = '#' as u8;
    }

    let mut output_file = File::create("output/13.B.txt").into_aoc_result()?;
    for row in grid {
        row[max_x + 1] = '\n' as u8;
        output_file.write_all(row).into_aoc_result()?;
    }

    Ok(0 as iAoc)
}
