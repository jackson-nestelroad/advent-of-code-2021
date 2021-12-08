use crate::common::{iAoc, AocResult, IntoAocResult};

fn parse_input(input: &str) -> AocResult<Vec<u8>> {
    input
        .split(',')
        .map(|num| num.parse::<u8>())
        .collect::<Result<_, _>>()
        .into_aoc_result()
}

const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

fn count_lanternfish(input: &str, days: usize) -> AocResult<iAoc> {
    let lanternfish = parse_input(input.trim())?;
    const FISH_TIMER: usize = 6;
    const NEW_FISH_TIMER: usize = 8;

    const LENGTH: usize = max(FISH_TIMER, NEW_FISH_TIMER) + 1;

    // Stores the frequency of each timer value.
    let mut timers: [u64; LENGTH] = [0; LENGTH];
    for fish in lanternfish {
        timers[fish as usize] += 1;
    }

    for _ in 0..days {
        let new_fish = timers[0];
        timers.rotate_left(1);
        timers[FISH_TIMER] += new_fish;
        timers[NEW_FISH_TIMER] = new_fish;
    }

    Ok(timers.iter().sum::<iAoc>())
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    count_lanternfish(input, 80)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    count_lanternfish(input, 256)
}
