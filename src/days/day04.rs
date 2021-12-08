use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use num::Integer;
use std::collections::HashMap;
use std::num::ParseIntError;

const BOARD_SIZE: usize = 5;

struct BingoBoard {
    // index_to_num: Vec<Vec<u32>>,
    num_to_index: HashMap<u32, (usize, usize)>,
    markings: [u8; BOARD_SIZE],
}

impl BingoBoard {
    pub fn mark(&mut self, num: u32) -> bool {
        match self.num_to_index.get(&num) {
            None => false,
            Some((row, col)) => {
                self.markings[*row] |= 1 << col;
                true
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        for marking in self.markings {
            if marking == (1 << BOARD_SIZE) - 1 {
                return true;
            }
        }
        for col in 0..BOARD_SIZE {
            let mut column_winner = true;
            for marking in self.markings {
                if marking & (1 << col) == 0 {
                    column_winner = false;
                    break;
                }
            }
            if column_winner {
                return column_winner;
            }
        }
        return false;
    }

    pub fn sum_unmarked(&self) -> u32 {
        self.num_to_index
            .iter()
            .filter_map(|(num, (row, col))| {
                if self.markings[*row] & (1 << col) == 0 {
                    Some(num)
                } else {
                    None
                }
            })
            .sum()
    }

    fn try_from_iter<'s, I>(input: I) -> AocResult<Self>
    where
        I: Iterator<Item = &'s str>,
    {
        let mut num_to_index: HashMap<u32, (usize, usize)> = HashMap::new();
        let row_iter = input
            .enumerate()
            .map::<Result<_, ParseIntError>, _>(|(row, line)| {
                Ok((row, line.split_whitespace().map(|n| n.parse::<u32>())))
            });
        for row in row_iter {
            let (row, num_iter) = row.into_aoc_result()?;
            for (col, num) in num_iter.enumerate() {
                num_to_index.insert(num.into_aoc_result()?, (row, col));
            }
        }
        Ok(BingoBoard {
            num_to_index,
            markings: [0; BOARD_SIZE],
        })
    }
}

fn parse_input(input: &str) -> AocResult<(Vec<u32>, Vec<BingoBoard>)> {
    let mut lines = input.lines();
    let numbers: Vec<u32> = lines
        .next()
        .into_aoc_result_msg("numbers list not found")?
        .split(',')
        .map(|n| n.parse::<u32>())
        .collect::<Result<_, _>>()
        .into_aoc_result()?;
    let mut boards: Vec<BingoBoard> = Vec::new();
    while lines.next().is_some() {
        boards.push(BingoBoard::try_from_iter(lines.by_ref().take(BOARD_SIZE))?);
    }
    Ok((numbers, boards))
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let (numbers, mut boards) = parse_input(input)?;
    for num in numbers {
        for board in &mut boards {
            if board.mark(num) {
                if board.is_winner() {
                    let score = board.sum_unmarked() as iAoc * num as iAoc;
                    return Ok(score);
                }
            }
        }
    }
    Err(AocError::new("no board won"))
}

fn check_bit(bits: &Vec<u64>, i: usize) -> bool {
    bits[i >> 6] & (1 << (i & 0x3F)) != 0
}

fn set_bit(bits: &mut Vec<u64>, i: usize) {
    bits[i >> 6] |= 1 << (i & 0x3F);
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let (numbers, mut boards) = parse_input(input)?;
    let mut winning_boards: Vec<u64> = vec![0; boards.len().div_ceil(&64)];
    let mut winning_board_count = 0;
    let all_but_one = boards.len() - 1;
    for num in numbers {
        for i in 0..boards.len() {
            let board = &mut boards[i];
            if !check_bit(&winning_boards, i) && board.mark(num) {
                if board.is_winner() {
                    if winning_board_count == all_but_one {
                        let score = board.sum_unmarked() as iAoc * num as iAoc;
                        return Ok(score);
                    } else {
                        winning_board_count += 1;
                        set_bit(&mut winning_boards, i);
                    }
                }
            }
        }
    }
    Err(AocError::new("all boards never won"))
}
