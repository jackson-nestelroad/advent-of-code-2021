use crate::common::{iAoC, Error};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone)]
struct BinaryDiagnosticData {
    pub entries: Vec<u32>,
    pub bits_per_line: usize,
}

impl FromStr for BinaryDiagnosticData {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<(usize, u32)> = match input
            .lines()
            .map::<Result<(usize, u32), ParseIntError>, _>(|line| {
                Ok((line.len(), u32::from_str_radix(line, 2)?))
            })
            .collect()
        {
            Err(err) => return Err(Error::new(err.to_string())),
            Ok(coll) => coll,
        };
        let bits_per_line = parsed
            .iter()
            .fold(usize::MIN, |max, (b_len, _)| max.max(*b_len));
        let entries = parsed.into_iter().map(|(_, num)| num).collect();
        Ok(BinaryDiagnosticData {
            entries,
            bits_per_line,
        })
    }
}

impl BinaryDiagnosticData {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn count_bits(&self) -> HashMap<u8, usize> {
        let mut bit_count = HashMap::new();
        for num in self.entries.iter() {
            for i in 0..self.bits_per_line {
                if num & (1 << i) != 0 {
                    *bit_count.entry(i as u8).or_insert(0) += 1;
                }
            }
        }
        bit_count
    }

    pub fn count_bits_at_pos(&self, i: usize) -> usize {
        self.entries
            .iter()
            .filter(|num| *num & (1 << i) != 0)
            .count()
    }

    pub fn filter<P>(self, predicate: P) -> Self
    where
        P: FnMut(&u32) -> bool,
    {
        BinaryDiagnosticData {
            entries: self.entries.into_iter().filter(predicate).collect(),
            bits_per_line: self.bits_per_line,
        }
    }
}

pub fn solve_a(input: &str) -> Result<iAoC, Error> {
    let data = BinaryDiagnosticData::from_str(input)?;
    let bit_count = data.count_bits();
    let majority = (data.len() as f64 / 2.0).ceil() as usize;
    let gamma = bit_count
        .into_iter()
        .filter(|(_, count)| count >= &majority)
        .fold(0u32, |result, (i, _)| result | (1 << i));
    let epsilon = !gamma & ((1 << data.bits_per_line) - 1);
    let result = gamma as iAoC * epsilon as iAoC;
    Ok(result)
}

pub fn solve_b(input: &str) -> Result<iAoC, Error> {
    let data = BinaryDiagnosticData::from_str(input)?;

    let bits = data.bits_per_line;
    let mut o2_candidates = data.clone();
    let mut co2_candidates = data;
    for i in (0..bits).rev() {
        let o2_finished = o2_candidates.len() == 1;
        let co2_finished = co2_candidates.len() == 1;

        if o2_finished && co2_finished {
            break;
        }

        if !o2_finished {
            let count_at_index = o2_candidates.count_bits_at_pos(i);
            let majority = (o2_candidates.len() as f64 / 2.0).ceil() as usize;
            let most_often_on = count_at_index >= majority;
            o2_candidates = o2_candidates.filter(|num| (num & (1 << i) != 0) == most_often_on);
        }
        if !co2_finished {
            let count_at_index = co2_candidates.count_bits_at_pos(i);
            let majority = (co2_candidates.len() as f64 / 2.0).ceil() as usize;
            let most_often_on = count_at_index >= majority;
            co2_candidates = co2_candidates.filter(|num| (num & (1 << i) != 0) == !most_often_on);
        }
    }

    if o2_candidates.len() != 1 || co2_candidates.len() != 1 {
        return Err(Error::new("value reduction did not complete"));
    }
    let o2_generator_rating = o2_candidates.entries[0];
    let co2_scrubber_rating = co2_candidates.entries[0];
    let result = o2_generator_rating as iAoC * co2_scrubber_rating as iAoC;
    Ok(result)
}
