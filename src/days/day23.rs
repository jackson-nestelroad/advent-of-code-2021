use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use num_traits::FromPrimitive;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
enum Amphipod {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

impl Amphipod {
    pub fn from_char(ch: char) -> Option<Amphipod> {
        match ch {
            'A' => Some(Self::Amber),
            'B' => Some(Self::Bronze),
            'C' => Some(Self::Copper),
            'D' => Some(Self::Desert),
            _ => None,
        }
    }

    pub fn energy(&self) -> usize {
        10usize.pow(*self as u32)
    }
}

/// A representation of the amphipod state, which can be encoded into 64 bits.
///
/// There are 11 spaces in the hallway, but 4 of them are invalid spaces because
/// they are directly outside of a room.
/// There are 4 rooms with 2 spaces each in part A and 4 spaces each in part B.
/// Each space has 5 potential states: empty, or one of four amphipods. These
/// 5 states can be represented as three bits.
///
/// (11 - 4) + (4 * 4) = 23^5 < 2^64
///
/// Thus, 64 bits can be used to represent every unique state of this system.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AmphipodState<const R: usize> {
    hallway: [Option<Amphipod>; 7],
    rooms: [[Option<Amphipod>; R]; 4],
}

impl<const R: usize> AmphipodState<R> {
    /// Number of possible states for an individual space.
    const SPACE_STATES: u64 = 5;

    pub fn new() -> Self {
        Self {
            hallway: [None; 7],
            rooms: [[None; R]; 4],
        }
    }

    pub fn goal() -> Self {
        Self {
            hallway: [None; 7],
            rooms: [
                [Some(Amphipod::Amber); R],
                [Some(Amphipod::Bronze); R],
                [Some(Amphipod::Copper); R],
                [Some(Amphipod::Desert); R],
            ],
        }
    }

    fn encode_space(space: Option<Amphipod>) -> u64 {
        match space {
            None => 0,
            Some(amp) => amp as u64 + 1,
        }
    }

    fn decode_space(space: u64) -> Option<Amphipod> {
        Amphipod::from_u64(space - 1)
    }

    /// Encodes the state into a 64-bit integer.
    pub fn encode(self) -> u64 {
        self.rooms
            .iter()
            .flatten()
            .rev()
            .chain(self.hallway.iter().rev())
            .fold(0u64, |acc, space| acc * 5 + Self::encode_space(*space))
    }

    /// Decodes a space encoding into the accessible data structure.
    pub fn decode(mut encoded: u64) -> Self {
        let mut it = std::iter::from_fn(move || {
            let space = encoded % Self::SPACE_STATES;
            encoded /= Self::SPACE_STATES;
            Some(Self::decode_space(space))
        });

        Self {
            hallway: [(); 7].map(|_| it.next().unwrap()),
            rooms: [(); 4].map(|_| [(); R].map(|_| it.next().unwrap())),
        }
    }

    /// Converts a hallway index to the actual X position in the hallway.
    fn hallway_x(index: usize) -> usize {
        if index < 2 {
            index
        } else if index >= 5 {
            index + 4
        } else {
            index + (index - 1)
        }
    }

    /// Converts an X position in the hallway to its corresponding array index.
    fn hallway_index(hallway_x: usize) -> usize {
        if hallway_x < 2 {
            hallway_x
        } else if hallway_x >= 9 {
            hallway_x - 4
        } else {
            hallway_x - hallway_x / 2
        }
    }

    /// Gets the X position in the hallway above the given room.
    fn room_x(room_index: usize) -> usize {
        2 * room_index + 2
    }

    /// Iterator over all of the next states of the current state.
    pub fn next_states<'a>(&'a self) -> impl Iterator<Item = (Self, usize)> + 'a {
        self.hallway_to_room().chain(self.room_to_hallway())
    }

    /// Checks if an amphipod can enter this room by assuring that the only amphipods
    /// in its target room (if any) are of the correct type.
    fn can_enter_room(&self, room_index: usize) -> bool {
        self.rooms[room_index].iter().all(|space| match space {
            None => true,
            Some(other_amp) => *other_amp as usize == room_index,
        })
    }

    /// Checks if an amphipod could move through the hallway, from `start_index` to
    /// `target_index`, which means all spaces between are empty.
    fn can_move_through_hallway(&self, start_index: usize, target_index: usize) -> bool {
        // This logic is a bit complicated because the target index does not actually
        // correspond to a position in the hallway array, because the space above a
        // room is not considered valid.
        //
        // By looking at which direction the amphipod is moving from, the correct
        // target index can be checked. This logic depends on the implementation
        // found in `hallway_index()`, which maps these invalid X positions to
        // existing indicies, specifically the hallway space to its immediate left.
        let (start_index, target_index) = match start_index.cmp(&target_index) {
            // Nothing can be in our way.
            Ordering::Equal => return true,
            // Coming from the left, need to check the target_index space.
            Ordering::Less => (start_index + 1, target_index),
            // Coming from the right, do not need to check the target_index space.
            Ordering::Greater => (target_index + 1, start_index - 1),
        };
        (start_index..=target_index).all(|x| self.hallway[x].is_none())
    }

    /// Calculates the distance between two X coordinates in the hallway.
    fn distance(a: usize, b: usize) -> usize {
        if a < b {
            b - a
        } else {
            a - b
        }
    }

    /// Generates all valid state changes for one amphipod in a hallway to its room.
    fn hallway_to_room<'a>(&'a self) -> impl Iterator<Item = (Self, usize)> + 'a {
        self.hallway
            .iter()
            .enumerate()
            // Filter out empty spaces.
            .filter_map(|(hallway_index, space)| space.map(|amp| (hallway_index, amp)))
            .filter_map(move |(hallway_index, amp)| {
                // First check that this amphipod can move into its room.
                let target_room = amp as usize;
                if !self.can_enter_room(target_room) {
                    return None;
                }

                // Then check that this amphipod can move to the space above its room.
                let hallway_x = Self::hallway_x(hallway_index);
                let target_room_x = Self::room_x(target_room);
                if !self.can_move_through_hallway(hallway_index, Self::hallway_index(target_room_x))
                {
                    return None;
                }

                // At this point, the move is valid, so select the first available
                // index in the room.
                let target_room_y = self.rooms[target_room]
                    .iter()
                    .rposition(|space| space.is_none())
                    .unwrap();

                // Travel through the hallway, then move into the room.
                let steps = Self::distance(hallway_x, target_room_x) + target_room_y + 1;

                // The cost for the move.
                let energy = steps * amp.energy();

                // Create the new state by copying the current one and swapping
                // the current position with the target position in the room.
                let mut new_state = *self;
                std::mem::swap(
                    &mut new_state.hallway[hallway_index],
                    &mut new_state.rooms[target_room][target_room_y],
                );

                Some((new_state, energy))
            })
    }

    /// Iterator over the reachable hallway spaces from the given position.
    fn reachable_hallway_spaces<'a>(
        &'a self,
        hallway_index: usize,
    ) -> impl Iterator<Item = usize> + 'a {
        // Move left and right, starting at the current hallway index, until
        // a non-empty space is hit.
        (0..hallway_index)
            .rev()
            .take_while(move |x| self.hallway[*x].is_none())
            .chain(
                (hallway_index..self.hallway.len()).take_while(move |x| self.hallway[*x].is_none()),
            )
    }

    /// Generates all valid state changes for one amphipod in a wrong room to the hallway.
    fn room_to_hallway<'a>(&'a self) -> impl Iterator<Item = (Self, usize)> + 'a {
        self.rooms
            .iter()
            .enumerate()
            // Filter out rooms that have only valid amphipods.
            .filter(move |(room_index, _)| !self.can_enter_room(*room_index))
            .flat_map(move |(room_index, room)| {
                // Get the position of the top-most amphipod, which is the only
                // one that can currently move out of the room.
                let room_x = Self::room_x(room_index);
                let (room_y, amp) = room
                    .iter()
                    .enumerate()
                    .find_map(|(y, space)| space.map(|amp| (y, amp)))
                    .unwrap();
                self.reachable_hallway_spaces(Self::hallway_index(room_x))
                    .map(move |hallway_index| {
                        let hallway_x = Self::hallway_x(hallway_index);
                        let steps = room_y + 1 + Self::distance(room_x, hallway_x);
                        let energy = steps * amp.energy();

                        let mut new_state = *self;
                        std::mem::swap(
                            &mut new_state.hallway[hallway_index],
                            &mut new_state.rooms[room_index][room_y],
                        );

                        (new_state, energy)
                    })
            })
    }

    /// Heuristic function for the A* algorithm.
    ///
    /// Calculates a lower bound for the energy cost from the current state to
    /// the goal state.
    ///
    /// Calculates the energy required for all amphipods in invalid positions
    /// to move directly to their goal position, regardless of obstacles.
    fn heuristic(&self) -> usize {
        // Cost of moving amphipods in the hallway to the space above their room.
        let hallway_to_above_room = self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(hallway_index, space)| space.map(|amp| (hallway_index, amp)))
            .map(move |(hallway_index, amp)| {
                let target_room = amp as usize;
                let hallway_x = Self::hallway_x(hallway_index);
                let target_room_x = Self::room_x(target_room);
                let steps = 1 + Self::distance(hallway_x, target_room_x);
                let energy = steps * amp.energy();
                energy
            })
            .sum::<usize>();
        // Cost of moving amphipods in the wrong room to the space above their room.
        let room_to_above_room = self
            .rooms
            .iter()
            .enumerate()
            .flat_map(|(room_index, room)| {
                let room_x = Self::room_x(room_index);
                room.iter()
                    .enumerate()
                    .rev()
                    .filter_map(|(room_y, space)| space.map(|amp| (room_y, amp)))
                    .skip_while(move |(_, amp)| room_index == *amp as usize)
                    .map(move |(room_y, amp)| {
                        let target_room = amp as usize;
                        let target_room_x = Self::hallway_x(target_room);
                        let hallway_steps = Self::distance(room_x, target_room_x).max(2);
                        let steps = room_y + 1 + hallway_steps;
                        let energy = steps * amp.energy();
                        energy
                    })
            })
            .sum::<usize>();

        // Cost of moving all amphipods above their room into their room.
        let above_room_to_room = self
            .rooms
            .iter()
            .enumerate()
            .map(
                |(room_index, room)| match room.iter().rposition(|space| space.is_none()) {
                    None => 0,
                    Some(first_open_y) => {
                        let steps = (first_open_y + 1) * first_open_y / 2;
                        let amp = Amphipod::from_usize(room_index).unwrap();
                        let energy = amp.energy() * steps;
                        energy
                    }
                },
            )
            .sum::<usize>();

        hallway_to_above_room + room_to_above_room + above_room_to_room
    }

    /// Implements the A* algorithm, searching for the shortest path from the
    /// start state to the goal state.
    pub fn solve(start: Self) -> AocResult<usize> {
        let encoded_goal = Self::goal().encode();
        let encoded_start = start.encode();

        let start_f_score = start.heuristic();
        let mut f_scores = HashMap::new();
        f_scores.insert(encoded_start, start_f_score);

        let mut g_scores = HashMap::new();
        g_scores.insert(encoded_start, 0);

        let mut open_set = BinaryHeap::new();
        open_set.push(Reverse((start_f_score, encoded_start)));

        while let Some(Reverse((f_score, encoded_state))) = open_set.pop() {
            let state = Self::decode(encoded_state);
            if encoded_state == encoded_goal {
                return Ok(f_score);
            }

            if f_score > f_scores.get(&encoded_state).copied().unwrap_or(usize::MAX) {
                continue;
            }

            let g_score = g_scores.get(&encoded_state).copied().unwrap();
            for (next_state, cost) in state.next_states() {
                let encoded_next_state = next_state.encode();
                let tentative_g_score = g_score + cost;
                let next_state_g_score = g_scores.entry(encoded_next_state).or_insert(usize::MAX);
                if tentative_g_score < *next_state_g_score {
                    let new_f_score = tentative_g_score + next_state.heuristic();
                    *f_scores.entry(encoded_next_state).or_default() = new_f_score;
                    *next_state_g_score = tentative_g_score;
                    open_set.push(Reverse((new_f_score, encoded_next_state)));
                }
            }
        }

        Err(AocError::new("no solution found"))
    }
}

impl<const R: usize> FromStr for AmphipodState<R> {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut state = Self::new();
        let mut lines = input.lines().skip(1);
        let hallway = lines.next().into_aoc_result()?;
        let mut offset = 0;
        for (i, space) in hallway[1..(hallway.len() - 1)].chars().enumerate() {
            match i {
                2 | 4 | 6 | 8 => offset += 1,
                _ => {
                    state.hallway[i - offset] = match space {
                        '.' => None,
                        ch => Amphipod::from_char(ch),
                    }
                }
            }
        }

        for (i, room_row) in lines.take(R).enumerate() {
            let mut chars = room_row[2..(2 + state.rooms.len() * 2)].chars();
            for r in 0..state.rooms.len() {
                chars.next();
                state.rooms[r][i] = match chars.next().into_aoc_result()? {
                    '.' => None,
                    ch => Amphipod::from_char(ch),
                }
            }
        }

        Ok(state)
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let state = AmphipodState::<2>::from_str(input)?;
    let result = AmphipodState::<2>::solve(state)?;
    Ok(result as iAoc)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let folded_state = AmphipodState::<2>::from_str(input)?;
    let mut unfolded_state = AmphipodState::<4>::new();

    const UNFOLDED_INPUT: [[Option<Amphipod>; 2]; 4] = [
        [Some(Amphipod::Desert), Some(Amphipod::Desert)],
        [Some(Amphipod::Copper), Some(Amphipod::Bronze)],
        [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
        [Some(Amphipod::Amber), Some(Amphipod::Copper)],
    ];
    for room_index in 0..4 {
        let mut it = std::iter::once(folded_state.rooms[room_index][0])
            .chain(UNFOLDED_INPUT[room_index].iter().copied())
            .chain(std::iter::once(folded_state.rooms[room_index][1]));
        unfolded_state.rooms[room_index] =
            unfolded_state.rooms[room_index].map(|_| it.next().unwrap());
    }

    let result = AmphipodState::<4>::solve(unfolded_state)?;
    Ok(result as iAoc)
}
