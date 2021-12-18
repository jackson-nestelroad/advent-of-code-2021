use crate::common::{iAoc, AocResult};

mod bits {
    use crate::common::{AocError, AocResult, IntoAocResult};
    use itertools::Itertools;
    use num::{FromPrimitive, Integer};

    pub type Input = Vec<u8>;

    pub fn parse_input(input: &str) -> AocResult<Input> {
        let mut result = Input::new();
        for mut chunk in input.trim().chars().chunks(2).into_iter() {
            let first = chunk
                .next()
                .and_then(|ch| ch.to_digit(16))
                .map(|val| val as u8);
            let second = chunk
                .next()
                .and_then(|ch| ch.to_digit(16))
                .map(|val| val as u8);
            result.push(match first {
                None => return Err(AocError::new("invalid hexadecimal byte")),
                Some(first) => match second {
                    None => first,
                    Some(second) => (first << 4) | second,
                },
            });
        }
        Ok(result)
    }

    #[repr(u8)]
    #[derive(FromPrimitive)]
    pub enum TypeId {
        Literal = 4,
        Sum = 0,
        Product = 1,
        Minimum = 2,
        Maximum = 3,
        GreaterThan = 5,
        LessThan = 6,
        EqualTo = 7,
    }

    pub struct Header {
        version: u8,
        type_id: TypeId,
    }

    pub struct Packet {
        header: Header,
        literal: u64,
        subpackets: Vec<Packet>,
    }

    impl Packet {
        pub fn new(header: Header) -> Self {
            Packet {
                header,
                literal: 0,
                subpackets: Vec::new(),
            }
        }

        pub fn sum_versions(&self) -> u64 {
            self.header.version as u64
                + self
                    .subpackets
                    .iter()
                    .fold(0u64, |sum, subpacket| sum + subpacket.sum_versions())
        }

        pub fn value(&self) -> AocResult<u64> {
            use TypeId::*;
            let mut subvalues = self.subpackets.iter().map(|subpacket| subpacket.value());
            match self.header.type_id {
                Literal => Ok(self.literal),
                Sum => subvalues.try_fold(0u64, |sum, value| Ok(sum + value?)),
                Product => subvalues.try_fold(1u64, |prod, value| Ok(prod * value?)),
                Minimum => subvalues.try_fold(u64::MAX, |min, value| match value {
                    Err(_) => value,
                    Ok(value) => Ok(if value < min { value } else { min }),
                }),
                Maximum => subvalues.try_fold(u64::MIN, |max, value| match value {
                    Err(_) => value,
                    Ok(value) => Ok(if value > max { value } else { max }),
                }),
                GreaterThan => {
                    let first = subvalues
                        .next()
                        .into_aoc_result_msg("missing first value")??;
                    let second = subvalues
                        .next()
                        .into_aoc_result_msg("missing second value")??;
                    Ok(if first > second { 1 } else { 0 })
                }
                LessThan => {
                    let first = subvalues
                        .next()
                        .into_aoc_result_msg("missing first value")??;
                    let second = subvalues
                        .next()
                        .into_aoc_result_msg("missing second value")??;
                    Ok(if first < second { 1 } else { 0 })
                }
                EqualTo => {
                    let first = subvalues
                        .next()
                        .into_aoc_result_msg("missing first value")??;
                    let second = subvalues
                        .next()
                        .into_aoc_result_msg("missing second value")??;
                    Ok(if first == second { 1 } else { 0 })
                }
            }
        }
    }

    pub struct Reader {
        input: Input,
        byte_index: usize,
        bit_index: usize,
    }

    impl Reader {
        pub fn new(input: Input) -> Self {
            Reader {
                input,
                byte_index: 0,
                bit_index: 8,
            }
        }

        pub fn global_bit_index(&self) -> usize {
            (self.byte_index << 3) + (8 - self.bit_index)
        }

        fn read_up_to_8(&mut self, num_bits: usize) -> Option<u8> {
            if num_bits == 0 || self.bit_index > 8 {
                None
            } else if self.bit_index >= num_bits {
                // Enough bits in the current byte.
                let diff = self.bit_index - num_bits;
                self.input
                    .get(self.byte_index)
                    .and_then(|val| {
                        Some((val & (((1usize << num_bits) - 1) << diff) as u8) >> diff)
                    })
                    .and_then(|val| {
                        self.bit_index = if diff == 0 {
                            self.byte_index += 1;
                            8
                        } else {
                            diff
                        };
                        Some(val)
                    })
            } else {
                // Not enough bits in the current byte, need the next byte.
                let diff = num_bits - self.bit_index;
                let not_diff = 8 - diff;
                self.input
                    .get(self.byte_index)
                    .copied()
                    .and_then(|first| {
                        self.byte_index += 1;
                        self.input.get(self.byte_index).and_then(|second| {
                            Some(
                                ((first & ((1 << self.bit_index) - 1)) << diff)
                                    | ((second & (((1 << diff) - 1) << not_diff)) >> not_diff),
                            )
                        })
                    })
                    .and_then(|val| {
                        self.bit_index = if not_diff == 0 {
                            self.byte_index += 1;
                            8
                        } else {
                            not_diff
                        };
                        Some(val)
                    })
            }
        }

        fn read_up_to_64(&mut self, num_bits: usize) -> Option<u64> {
            if num_bits == 0 {
                None
            } else {
                let (div, rem) = num_bits.div_mod_floor(&8);

                let has_extra_byte = rem != 0;

                let mut output: u64 = 0;
                for _ in 0..div {
                    output <<= 8;
                    output |= self.read_up_to_8(8)? as u64;
                }

                if has_extra_byte {
                    output <<= rem;
                    output |= self.read_up_to_8(rem)? as u64;
                }

                Some(output)
            }
        }

        fn read_packet(&mut self) -> AocResult<Packet> {
            let mut packet = Packet::new(self.read_header()?);
            match packet.header.type_id {
                TypeId::Literal => packet.literal = self.read_literal()?,
                _ => packet.subpackets = self.read_operator()?,
            };

            Ok(packet)
        }

        fn read_header(&mut self) -> AocResult<Header> {
            let version = self
                .read_up_to_8(3)
                .into_aoc_result_msg("missing 3-bit version")?;
            let type_id = self
                .read_up_to_8(3)
                .into_aoc_result_msg("missing 3-bit type id")?;
            let type_id = TypeId::from_u8(type_id).into_aoc_result_msg("invalid type id")?;
            Ok(Header { version, type_id })
        }

        fn read_literal(&mut self) -> AocResult<u64> {
            let mut literal: u64 = 0;
            let mut more_to_read = true;
            while more_to_read {
                let next_bits = self
                    .read_up_to_8(5)
                    .into_aoc_result_msg("missing 5-bit literal chunk")?;
                more_to_read = next_bits & (1 << 4) != 0;
                literal <<= 4;
                literal |= (next_bits & ((1 << 4) - 1)) as u64;
            }

            Ok(literal)
        }

        fn read_operator(&mut self) -> AocResult<Vec<Packet>> {
            let length_type_id = self
                .read_up_to_8(1)
                .into_aoc_result_msg("missing 1-bit length type id")?;

            let mut subpackets = Vec::new();

            if length_type_id == 0 {
                let total_subpacket_length = self
                    .read_up_to_64(15)
                    .into_aoc_result_msg("missing 15-bit total subpacket length")?;

                let end_index = self.global_bit_index() + total_subpacket_length as usize;
                while self.global_bit_index() < end_index {
                    subpackets.push(self.read_packet()?);
                }
            } else {
                let num_subpackets = self
                    .read_up_to_64(11)
                    .into_aoc_result_msg("missing 11-bit subpacket number")?;

                for _ in 0..num_subpackets {
                    subpackets.push(self.read_packet()?);
                }
            }

            Ok(subpackets)
        }

        pub fn read(&mut self) -> AocResult<Packet> {
            self.read_packet()
        }
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let input = bits::parse_input(input)?;
    let mut reader = bits::Reader::new(input);
    let packet = reader.read()?;
    let result = packet.sum_versions();
    Ok(result as iAoc)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let input = bits::parse_input(input)?;
    let mut reader = bits::Reader::new(input);
    let packet = reader.read()?;
    let result = packet.value()?;
    Ok(result as iAoc)
}
