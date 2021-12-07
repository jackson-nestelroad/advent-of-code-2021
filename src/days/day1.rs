use crate::common::{iAoc, AocResult, IntoAocResult};

fn read_depths(input: &str) -> AocResult<Vec<i32>> {
    input
        .lines()
        .map(|depth| depth.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .into_aoc_result()
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let depths: Vec<i32> = read_depths(input)?;
    let result = depths
        .iter()
        .zip(depths.iter().skip(1))
        .fold(
            0,
            |result, (prev, next)| if prev < next { result + 1 } else { result },
        );
    Ok(result)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let depths: Vec<i32> = read_depths(input)?;
    let windows: Vec<i32> = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    let result = windows
        .iter()
        .zip(windows.iter().skip(1))
        .fold(
            0,
            |result, (prev, next)| if prev < next { result + 1 } else { result },
        );
    Ok(result)
}
