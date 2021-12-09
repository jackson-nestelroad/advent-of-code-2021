use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use std::collections::HashSet;

/// Each segment of a seven segment display can be mapped to a single bit.
/// Thus, an entire display can be stored as an 8-bit integer (or byte).
/// A bit is 1 if the segment is on and 0 if it is off.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum SevenSegment {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
    E = 1 << 4,
    F = 1 << 5,
    G = 1 << 6,
}

impl SevenSegment {
    /// Maps a digit to its seven segment display.
    const DIGIT_DISPLAY: [u8; 10] = [
        0b1110111, // 0
        0b0100100, // 1
        0b1011101, // 2
        0b1101101, // 3
        0b0101110, // 4
        0b1101011, // 5
        0b1111011, // 6
        0b0100101, // 7
        0b1111111, // 8
        0b1101111, // 9
    ];

    /// Converts each character to a single bit, creating a seven segment bit string.
    pub fn from_str(input: &str) -> AocResult<u8> {
        use SevenSegment::*;
        input
            .chars()
            .map(|ch| match ch {
                'a' => Ok(A),
                'b' => Ok(B),
                'c' => Ok(C),
                'd' => Ok(D),
                'e' => Ok(E),
                'f' => Ok(F),
                'g' => Ok(G),
                _ => return Err(AocError::new("invalid character for seven segment display")),
            })
            .try_fold(0, |acc, seg| Ok(acc | seg? as u8))
    }
}

/// Segments are given as input using some permutation "abcdefg", with each letter being optional.
/// The `SevenSegment` enum maps a single letter to a bit, so each string becomes a bit string.
///
/// For example:
///     "bc" => 0b0000110
///     "abcefg" => 0b1110111
///
/// The length of a string becomes the number of 1 bits in the bit string.
///
/// The problem is to correctly map bit positions from the `key` to form the values in `DIGIT_DISPLAY`,
/// and then to read `reading `with that same mapping.
///
/// For example:
///     Consider "bc" => 0b000110.
///     This display can only be 0b0100100, so bits B and C can be mapped to {C, F}.
///     The key must be further expored to figure out which bit maps to C and which maps to F.
struct SegmentWiring {
    key: Vec<u8>,
    reading: Vec<u8>,
}

impl SegmentWiring {
    pub fn read(&self, mapping: [u8; 7]) -> AocResult<u64> {
        self.reading
            .iter()
            .map(|output| {
                let mut result = 0;
                for bit in 0..7 {
                    if output & (1 << bit) != 0 {
                        result |= mapping[bit];
                    }
                }

                // At this point, result is a proper digit display, we just need to map
                // it back to a 0-9 digit.

                SevenSegment::DIGIT_DISPLAY
                    .iter()
                    .position(|&display| display == result)
                    .into_aoc_result_msg("failed to map output to a proper digit")
            })
            .try_fold(0u64, |acc, digit| Ok(10 * acc + digit? as u64))
    }
}

fn parse_input(input: &str) -> AocResult<Vec<SegmentWiring>> {
    input
        .lines()
        .map::<AocResult<_>, _>(|line| {
            let (input, output) = line.split_once(" | ").into_aoc_result()?;
            Ok(SegmentWiring {
                key: input
                    .split(' ')
                    .map(|s| SevenSegment::from_str(s))
                    .collect::<Result<_, _>>()?,
                reading: output
                    .split(' ')
                    .map(|s| SevenSegment::from_str(s))
                    .collect::<Result<_, _>>()?,
            })
        })
        .collect::<AocResult<Vec<_>>>()
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    const DESIRED_DIGITS: [usize; 4] = [1, 4, 7, 8];

    // The number of bits that should be set for numbers we're interested in.
    let desired_count_ones: HashSet<u32> = DESIRED_DIGITS
        .iter()
        .map(|digit| SevenSegment::DIGIT_DISPLAY[*digit].count_ones())
        .collect();

    let wirings = parse_input(input)?;
    let result: usize = wirings
        .into_iter()
        .map(|wiring| {
            wiring
                .reading
                .into_iter()
                .filter(|out| desired_count_ones.contains(&out.count_ones()))
                .count()
        })
        .sum();
    Ok(result as iAoc)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    // Maps the number of bits set to the potential digits it could be.
    let mut ones_count_to_digit: Vec<Vec<usize>> = std::iter::repeat(vec![]).take(8).collect();
    for (digit, display) in SevenSegment::DIGIT_DISPLAY.iter().enumerate() {
        ones_count_to_digit[display.count_ones() as usize].push(digit);
    }

    let wirings = parse_input(input)?;
    let mut result: iAoc = 0;

    for wiring in &wirings {
        // Maps a single segment bit to the potential segments it can be,
        // represented by a bit string.
        let mut segment_mapping: [u8; 7] = [0b1111111; 7];
        for key in &wiring.key {
            // For each bit, update the potential the segment mapping.
            // If the bit is on, then it must map to the union (bitwise OR) of potential bit mappings.
            // If the bit is off, then it must not map to the union of potential bit mappings,
            // so we take the inverse of the union, (NOT (bitwise AND)).
            let (potential_if_active, mut potential_if_inactive) = ones_count_to_digit
                [key.count_ones() as usize]
                .iter()
                .map(|digit| SevenSegment::DIGIT_DISPLAY[*digit])
                .fold((0, 0b1111111), |(active, inactive), display| {
                    (active | display, inactive & display)
                });
            potential_if_inactive = !potential_if_inactive & 0b1111111;

            for bit in 0..7 {
                let entry = &mut segment_mapping[bit];
                *entry &= if key & (1 << bit) != 0 {
                    potential_if_active
                } else {
                    potential_if_inactive
                };
            }
        }

        // At this point, the key is properly mapped to be read. However, it is not guaranteed
        // that each value in segment_mapping is only one bit. This is because some value in the map
        // may still contain a bit that is already taken (the only bit in some other entry) by another
        // segment bit.
        //
        // Thus, we find all of the taken bits and unset them on values that are not finalized.

        let mut taken_bits = segment_mapping
            .iter()
            .filter(|mapping| mapping.count_ones() == 1)
            .fold(0, |acc, mapping| acc | mapping);

        for entry in &mut segment_mapping {
            if entry.count_ones() != 1 {
                *entry &= !taken_bits & 0b1111111;
                taken_bits |= *entry;
            }
        }

        // Read back the display and add it to the result.
        result += wiring.read(segment_mapping)?;
    }

    Ok(result as iAoc)
}
