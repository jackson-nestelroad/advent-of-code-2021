use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::str::FromStr;

#[derive(Clone, Copy)]
#[repr(u8)]
enum SeaCucumber {
    East,
    South,
}

struct SeaCucumberHerds {
    data: Vec<Option<SeaCucumber>>,
    height: usize,
    width: usize,
}

impl SeaCucumberHerds {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            data: vec![None; height * width],
            height,
            width,
        }
    }

    pub fn step(mut self) -> (Self, bool) {
        let mut next = SeaCucumberHerds::new(self.height, self.width);
        let mut changed = false;

        // First move all east sea cucumbers.
        for y in 0..self.height {
            let row_start = y * self.width;
            let row = &mut self.data[row_start..(row_start + self.width)];

            // Find the first empty space in the current row.
            // If it does not exist, the row is in gridlock.
            let first_empty_x = row.iter().position(|space| space.is_none());
            if let Some(first_empty_x) = first_empty_x {
                // Found an empty space, move left across the entire row and
                // move sea cucumbers accordingly.

                // Condition stating that the next east sea cucumber can move.
                let mut can_move = true;
                let mut prev = first_empty_x;
                for x in (0..first_empty_x)
                    .rev()
                    .chain(((first_empty_x + 1)..self.width).rev())
                {
                    match row[x] {
                        Some(SeaCucumber::East) => {
                            if can_move {
                                // East sea cucumber can move.

                                changed = true;
                                can_move = false;

                                // Move sea cucumber.
                                next.data[row_start + prev] = Some(SeaCucumber::East);

                                // Also update this sea cucumber's location in the current
                                // state, so that south sea cucumbers see its updated
                                // position.
                                //
                                // Since we are iterating from right-to-left and do not
                                // repeat any values, this sea cucumber will surely not
                                // move again.
                                row[x] = None;
                                row[prev] = Some(SeaCucumber::East);
                            } else {
                                // East sea cucumber cannot move, keep it in same position.
                                next.data[row_start + x] = Some(SeaCucumber::East);
                            }
                        }
                        Some(_) => {
                            can_move = false;
                        }
                        None => {
                            can_move = true;
                        }
                    }
                    prev = x;
                }
            } else {
                // Gridlock, copy over sea cucumbers to new state.
                for x in row.iter().enumerate().filter_map(|(x, space)| {
                    if let Some(SeaCucumber::East) = space {
                        Some(x)
                    } else {
                        None
                    }
                }) {
                    next.data[row_start + x] = Some(SeaCucumber::East);
                }
            }
        }

        // Now move all south sea cucumbers.
        for x in 0..self.width {
            // Find the first empty space in the current column.
            // If it does not exist, the row is in gridlock.
            let first_empty_y =
                (0..self.height).position(|y| self.data[y * self.width + x].is_none());
            if let Some(first_empty_y) = first_empty_y {
                // Found an empty space, move up across the entire column and
                // move sea cucumbers accordingly.

                // Condition stating that the next south sea cucumber can move.
                let mut can_move = true;
                let mut prev_index = first_empty_y * self.width + x;
                for y in (0..first_empty_y)
                    .rev()
                    .chain(((first_empty_y + 1)..self.height).rev())
                {
                    let current_index = y * self.width + x;
                    match self.data[current_index] {
                        Some(SeaCucumber::South) => {
                            if can_move {
                                // South sea cucumber can move.

                                changed = true;
                                can_move = false;

                                // Move sea cucumber.
                                next.data[prev_index] = Some(SeaCucumber::South);
                            } else {
                                // South sea cucumber cannot move, keep it in same position.
                                next.data[current_index] = Some(SeaCucumber::South);
                            }
                        }
                        Some(_) => {
                            can_move = false;
                        }
                        None => {
                            can_move = true;
                        }
                    }
                    prev_index = current_index;
                }
            } else {
                // Gridlock, copy over sea cucumbers to new state.
                for index in (0..self.height).filter_map(|y| {
                    let index = y * self.width + x;
                    if let Some(SeaCucumber::South) = self.data[index] {
                        Some(index)
                    } else {
                        None
                    }
                }) {
                    next.data[index] = Some(SeaCucumber::South);
                }
            }
        }

        (next, changed)
    }
}

impl Display for SeaCucumberHerds {
    fn fmt(&self, f: &mut Formatter) -> DisplayResult {
        let mut index = 0;
        for _ in 0..self.height {
            for _ in 0..self.width {
                let ch = match self.data[index] {
                    Some(SeaCucumber::East) => '>',
                    Some(SeaCucumber::South) => 'v',
                    None => '.',
                };
                write!(f, "{}", ch)?;
                index += 1;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl FromStr for SeaCucumberHerds {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines();

        let height = lines.clone().count();
        let width = lines.clone().next().into_aoc_result_msg("no rows")?.len();
        let mut herds = Self::new(height, width);

        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate() {
                herds.data[y * width + x] = match ch {
                    '>' => Some(SeaCucumber::East),
                    'v' => Some(SeaCucumber::South),
                    '.' => None,
                    _ => return Err(AocError::new("invalid character")),
                }
            }
        }

        Ok(herds)
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let mut herds = SeaCucumberHerds::from_str(input)?;
    let mut steps = 0;
    loop {
        steps += 1;
        let (updated_herds, changed) = herds.step();
        if !changed {
            break;
        }
        herds = updated_herds;
    }
    Ok(steps as iAoc)
}

pub fn solve_b(_: &str) -> AocResult<iAoc> {
    Ok(0)
}
