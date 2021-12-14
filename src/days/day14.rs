use crate::common::{iAoc, AocResult, IntoAocResult};
use std::collections::HashMap;

struct PolymerData {
    template: String,
    insertion_rules: HashMap<(char, char), char>,
}

/// Creates an iterator over slices of the source string in overlapping windows
/// of a dedicated size.
fn char_windows<'a>(src: &'a str, size: usize) -> impl Iterator<Item = &'a str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(size - 1)
            .next()
            .map(|(to, c)| &src[from..(from + to + c.len_utf8())])
    })
}

impl PolymerData {
    pub fn from_str(input: &str) -> AocResult<PolymerData> {
        let mut lines = input.lines();
        let template = lines.next().into_aoc_result()?.to_owned();
        let mut insertion_rules = HashMap::new();
        for line in lines.skip(1) {
            let (existing, between) = line.split_once(" -> ").into_aoc_result()?;
            let mut chars = existing.chars();
            insertion_rules.insert(
                (
                    chars.next().into_aoc_result()?,
                    chars.next().into_aoc_result()?,
                ),
                between.chars().next().into_aoc_result()?,
            );
        }
        Ok(PolymerData {
            template,
            insertion_rules,
        })
    }

    pub fn transform(&self, steps: usize) -> AocResult<HashMap<char, usize>> {
        // Maps a pair to the number of times it occurs.
        let mut pair_occurrences: HashMap<(char, char), usize> = HashMap::new();

        // Load all initial pairs into the map.
        for pair in char_windows(&self.template, 2) {
            let mut chars = pair.chars();
            *pair_occurrences
                .entry((
                    chars.next().into_aoc_result()?,
                    chars.next().into_aoc_result()?,
                ))
                .or_insert(0) += 1;
        }

        for _ in 0..steps {
            // Build the next map of pair occurrences using the previous map.
            let mut next_pair_occurrences = HashMap::new();

            for (pair, count) in pair_occurrences {
                match self.insertion_rules.get(&pair) {
                    None => *next_pair_occurrences.entry(pair).or_insert(0) += count,
                    Some(insert) => {
                        *next_pair_occurrences.entry((pair.0, *insert)).or_insert(0) += count;
                        *next_pair_occurrences.entry((*insert, pair.1)).or_insert(0) += count;
                    }
                }
            }

            pair_occurrences = next_pair_occurrences;
        }

        // For each pair, mark the first character in the pair as an occurrence.
        //
        // This successfully counts each character once because every character, except for the
        // first and last, are duplicated in two pairs.
        //
        // Because we only count the first character in each pair, we must manually add one to
        // the count for the last character in the original string.
        let mut occurrences = HashMap::new();
        for ((first, _), count) in pair_occurrences {
            *occurrences.entry(first).or_insert(0) += count;
        }

        *occurrences
            .entry(self.template.chars().last().into_aoc_result()?)
            .or_insert(0) += 1;

        Ok(occurrences)
    }
}

fn solve(input: &str, steps: usize) -> AocResult<iAoc> {
    let data = PolymerData::from_str(input)?;

    let occurrences = data.transform(steps)?;

    let (_, max_count) = occurrences
        .iter()
        .max_by_key(|&(_, count)| count)
        .into_aoc_result()?;
    let (_, min_count) = occurrences
        .iter()
        .min_by_key(|&(_, count)| count)
        .into_aoc_result()?;

    let result = max_count - min_count;
    Ok(result as iAoc)
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    solve(input, 10)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    solve(input, 40)
}
