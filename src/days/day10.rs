use crate::common::{iAoc, AocError, AocResult, IntoAocResult};

#[derive(PartialEq, Eq)]
enum ChunkDelimiter {
    Round,
    Square,
    Curly,
    Angled,
}

impl ChunkDelimiter {
    fn syntax_error_score(&self) -> iAoc {
        use ChunkDelimiter::*;
        match self {
            Round => 3,
            Square => 57,
            Curly => 1197,
            Angled => 25137,
        }
    }

    fn auto_correct_score(&self) -> iAoc {
        use ChunkDelimiter::*;
        match self {
            Round => 1,
            Square => 2,
            Curly => 3,
            Angled => 4,
        }
    }

    fn from_begin(ch: char) -> Option<Self> {
        use ChunkDelimiter::*;
        match ch {
            '(' => Some(Round),
            '[' => Some(Square),
            '{' => Some(Curly),
            '<' => Some(Angled),
            _ => None,
        }
    }

    fn from_end(ch: char) -> Option<Self> {
        use ChunkDelimiter::*;
        match ch {
            ')' => Some(Round),
            ']' => Some(Square),
            '}' => Some(Curly),
            '>' => Some(Angled),
            _ => None,
        }
    }
}

fn corrupted_syntax_score(line: &str) -> AocResult<iAoc> {
    let mut score: iAoc = 0;
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => match stack.pop() {
                None => (),
                Some(expected_char) => {
                    let expected = ChunkDelimiter::from_begin(expected_char).into_aoc_result()?;
                    let found = ChunkDelimiter::from_end(ch).into_aoc_result()?;
                    if expected != found {
                        score += found.syntax_error_score();
                    }
                }
            },
            _ => return Err(AocError::new("unexpected char found")),
        }
    }
    Ok(score)
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let result = input
        .lines()
        .map(|line| corrupted_syntax_score(line))
        .collect::<Result<Vec<iAoc>, _>>()?
        .iter()
        .sum();
    Ok(result)
}

fn is_corrupted(line: &str) -> bool {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => match stack.pop() {
                None => (),
                Some(expected_char) => {
                    // We have only looked at valid begin and end delimiters, so unwrap is safe here.
                    let expected = ChunkDelimiter::from_begin(expected_char).unwrap();
                    let found = ChunkDelimiter::from_end(ch).unwrap();
                    if expected != found {
                        return true;
                    }
                }
            },
            _ => return true,
        }
    }
    false
}

fn incomplete_correction_score(line: &str) -> AocResult<iAoc> {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            // We have asserted that the line is not corrupted at this point, so no need to check.
            ')' | ']' | '}' | '>' => {
                stack.pop();
            }
            _ => return Err(AocError::new("unexpected char found")),
        }
    }
    stack.into_iter().rev().try_fold(0u64, |score, expected| {
        Ok(score * 5
            + ChunkDelimiter::from_begin(expected)
                .into_aoc_result()?
                .auto_correct_score())
    })
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let mut scores = input
        .lines()
        .filter(|line| !is_corrupted(line))
        .map(|line| incomplete_correction_score(line))
        .collect::<Result<Vec<iAoc>, _>>()?;
    scores.sort();
    let mid = scores.len() / 2;
    let result = scores[mid];
    Ok(result)
}
