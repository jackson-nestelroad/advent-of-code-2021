use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use std::collections::VecDeque;
use std::str::FromStr;

struct DumboEnergyLevels {
    map: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl DumboEnergyLevels {
    pub fn new(map: Vec<Vec<u32>>) -> Self {
        let height = map.len();
        let width = map.first().map(|row| row.len()).unwrap_or(0);
        DumboEnergyLevels { map, height, width }
    }

    pub fn size(&self) -> usize {
        self.height * self.width
    }

    pub fn step(&mut self) -> usize {
        let mut to_flash = VecDeque::new();
        for (y, row) in self.map.iter_mut().enumerate() {
            for (x, energy_level) in row.iter_mut().enumerate() {
                *energy_level += 1;
                if *energy_level > 9 {
                    to_flash.push_back((x, y));
                }
            }
        }

        let mut flashes = 0;
        while !to_flash.is_empty() {
            let (x, y) = to_flash.pop_front().unwrap();
            let energy_level = &mut self.map[y][x];
            if *energy_level > 9 {
                flashes += 1;
                *energy_level = 0;
                for (dx, dy) in NEIGHBORS.iter() {
                    let neighbor_y = y.overflowing_add(*dy as usize).0;
                    let neighbor_x = x.overflowing_add(*dx as usize).0;
                    self.map
                        .get_mut(neighbor_y)
                        .and_then(|row| row.get_mut(neighbor_x))
                        .map(|neighbor_energy| {
                            if *neighbor_energy != 0 {
                                *neighbor_energy += 1;
                                if *neighbor_energy > 9 {
                                    to_flash.push_back((neighbor_x, neighbor_y));
                                }
                            }
                        });
                }
            }
        }

        flashes
    }
}

impl FromStr for DumboEnergyLevels {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).into_aoc_result())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;
        Ok(DumboEnergyLevels::new(map))
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let mut octopi = DumboEnergyLevels::from_str(input)?;

    let mut total_flashes: iAoc = 0;
    for _ in 0..100 {
        total_flashes += octopi.step() as iAoc;
    }
    Ok(total_flashes)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let mut octopi = DumboEnergyLevels::from_str(input)?;
    let total = octopi.size();

    let mut step: iAoc = 0;
    loop {
        step += 1;

        if octopi.step() == total {
            break;
        }
    }
    Ok(step)
}
