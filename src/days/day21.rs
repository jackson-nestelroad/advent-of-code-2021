use crate::common::{iAoc, AocResult, IntoAocResult};
use itertools::Itertools;

#[derive(Clone, Copy)]
struct PlayerState {
    pos: u8,
    points: usize,
}

impl PlayerState {
    pub fn new(pos: u8) -> Self {
        PlayerState { pos, points: 0 }
    }
}

struct PracticeDiracDie {
    p1: PlayerState,
    p2: PlayerState,
    rolls: usize,
    p1_turn: bool,
}

impl PracticeDiracDie {
    const SPACES: u8 = 10;
    const ROLLS_PER_TURN: usize = 3;
    const MAX_ROLL: usize = 100;

    pub fn new(p1_pos: u8, p2_pos: u8) -> Self {
        PracticeDiracDie {
            p1: PlayerState::new((p1_pos - 1) % Self::SPACES),
            p2: PlayerState::new((p2_pos - 1) % Self::SPACES),
            rolls: 0,
            p1_turn: true,
        }
    }

    fn done(&self) -> bool {
        self.p1.points >= 1000 || self.p2.points >= 1000
    }

    pub fn loser(&self) -> Option<PlayerState> {
        if self.p1.points >= 1000 {
            Some(self.p2)
        } else if self.p2.points >= 1000 {
            Some(self.p1)
        } else {
            None
        }
    }

    pub fn times_rolled(&self) -> usize {
        self.rolls
    }

    fn next_player(&mut self) -> &mut PlayerState {
        if self.p1_turn {
            &mut self.p1
        } else {
            &mut self.p2
        }
    }

    fn roll(&mut self) -> usize {
        let total = Self::ROLLS_PER_TURN
            + (0..Self::ROLLS_PER_TURN)
                .map(|i| (self.rolls + i) % Self::MAX_ROLL)
                .sum::<usize>();
        self.rolls += Self::ROLLS_PER_TURN;
        total
    }

    pub fn play(&mut self) {
        while !self.done() {
            let roll = self.roll();
            let player = self.next_player();
            player.pos = ((player.pos as usize + roll) % (Self::SPACES as usize)) as u8;
            player.points += (player.pos + 1) as usize;
            self.p1_turn = !self.p1_turn;
        }
    }
}

fn parse_positions(input: &str) -> AocResult<(u8, u8)> {
    let mut lines = input.lines();
    let first = lines
        .next()
        .into_aoc_result()?
        .split_once(": ")
        .into_aoc_result()?
        .1
        .parse::<u8>()
        .into_aoc_result()?;
    let second = lines
        .next()
        .into_aoc_result()?
        .split_once(": ")
        .into_aoc_result()?
        .1
        .parse::<u8>()
        .into_aoc_result()?;
    Ok((first, second))
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let (p1, p2) = parse_positions(input)?;
    let mut game = PracticeDiracDie::new(p1, p2);
    game.play();
    let losing_score =
        game.loser().into_aoc_result_msg("no losing player")?.points * game.times_rolled();
    Ok(losing_score as iAoc)
}

/// A bitwise representation of the game state.
///
/// 19 bits are used to represent the game state.
///     AAAAA_BBBBB_CCCC_DDDD_E
///     A = Player 1 points
///     B = Player 2 points
///     C = Player 1 position
///     D = Player 2 position
///     E = Next turn
///
/// Five bits can be used to represent the number of points because the game
/// ends when one player reaches at least 21 points. The maximum number of
/// points that can be earned before the game is detected as over is 23,
/// which is earned by a player with 20 points who roles a 3.
///
/// Four bits can be used to represent the position of a player because the
/// board only has 10 possible spaces.
///
/// A single bit can be used to represent whose turn it is.
///
/// The points are stored first because they represent how far along the game
/// is, helping optimize iteration through all game states and checking game
/// winners.
#[derive(Clone, Copy, PartialEq, Eq)]
struct GameState(u32);

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Player {
    Player1 = 0,
    Player2 = 1,
}

impl GameState {
    const GAME_STATE: u32 = 0b11111_11111_1111_1111_1;
    const P1_POINTS: u32 = 0b11111_00000_0000_0000_0;
    const P2_POINTS: u32 = 0b00000_11111_0000_0000_0;
    const P1_POSITION: u32 = 0b00000_00000_1111_0000_0;
    const P2_POSITION: u32 = 0b00000_00000_0000_1111_0;
    const NEXT_PLAYER: u32 = 0b00000_00000_0000_0000_1;

    const P1_POINTS_SHIFT: u8 = 14;
    const P2_POINTS_SHIFT: u8 = 9;
    const P1_POSITION_SHIFT: u8 = 5;
    const P2_POSITION_SHIFT: u8 = 1;

    const SPACES: u32 = 10;
    const WINNING_SCORE: u32 = 21;
    const P1_WINS: u32 = Self::WINNING_SCORE << Self::P1_POINTS_SHIFT;
    const P2_WINS: u32 = Self::WINNING_SCORE << Self::P2_POINTS_SHIFT;

    pub fn next_player(&self) -> Player {
        if self.0 & Self::NEXT_PLAYER == 0 {
            Player::Player1
        } else {
            Player::Player2
        }
    }

    pub fn flip_turn(&mut self) {
        self.0 ^= 1;
    }

    pub fn get_winner(&self) -> Option<Player> {
        if self.0 >= Self::P1_WINS {
            Some(Player::Player1)
        } else if (self.0 & !Self::P1_POINTS) >= Self::P2_WINS {
            Some(Player::Player2)
        } else {
            None
        }
    }

    pub fn get_points(&self, player: Player) -> u32 {
        match player {
            Player::Player1 => (self.0 & Self::P1_POINTS) >> Self::P1_POINTS_SHIFT,
            Player::Player2 => (self.0 & Self::P2_POINTS) >> Self::P2_POINTS_SHIFT,
        }
    }

    pub fn get_position(&self, player: Player) -> u32 {
        match player {
            Player::Player1 => (self.0 & Self::P1_POSITION) >> Self::P1_POSITION_SHIFT,
            Player::Player2 => (self.0 & Self::P2_POSITION) >> Self::P2_POSITION_SHIFT,
        }
    }

    pub fn move_player(&mut self, player: Player, pos: u32) -> u32 {
        let new_pos = (self.get_position(player) + pos) % Self::SPACES;
        match player {
            Player::Player1 => {
                self.0 &= !Self::P1_POSITION;
                self.0 |= new_pos << Self::P1_POSITION_SHIFT;
            }
            Player::Player2 => {
                self.0 &= !Self::P2_POSITION;
                self.0 |= new_pos << Self::P2_POSITION_SHIFT;
            }
        }
        new_pos
    }

    pub fn increase_points(&mut self, player: Player, add: u32) -> u32 {
        let new_points = self.get_points(player) + add;
        match player {
            Player::Player1 => {
                self.0 &= !Self::P1_POINTS;
                self.0 |= new_points << Self::P1_POINTS_SHIFT;
            }
            Player::Player2 => {
                self.0 &= !Self::P2_POINTS;
                self.0 |= new_points << Self::P2_POINTS_SHIFT;
            }
        }
        new_points
    }
}

struct DiracDie {
    // Vector of all possible game states.
    // Maps a game state to the number of universes in that state.
    games: Vec<usize>,
}

impl DiracDie {
    const ROLLS_PER_TURN: usize = 3;
    const MIN_ROLL: u32 = 1;
    const MAX_ROLL: u32 = 3;

    pub fn new(p1_pos: u8, p2_pos: u8) -> Self {
        let mut result = DiracDie {
            games: vec![0; GameState::GAME_STATE as usize],
        };

        // Create initial game.
        let mut initial_state = GameState(0);
        initial_state.move_player(Player::Player1, (p1_pos - 1) as u32);
        initial_state.move_player(Player::Player2, (p2_pos - 1) as u32);
        result.games[initial_state.0 as usize] = 1;

        result
    }

    fn possible_rolls(&self) -> impl Iterator<Item = Vec<u32>> {
        (0..Self::ROLLS_PER_TURN)
            .map(|_| (Self::MIN_ROLL..=Self::MAX_ROLL))
            .multi_cartesian_product()
    }

    pub fn play(&mut self) {
        let mut done = false;
        while !done {
            done = true;
            for game in 0..self.games.len() {
                let universe_count = self.games[game];
                if universe_count != 0 {
                    let state = GameState(game as u32);

                    // This game already has a winner, no need to progress farther.
                    if state.get_winner().is_some() {
                        continue;
                    }

                    // Split off on all possible dice rolls.
                    done = false;
                    for roll in self.possible_rolls() {
                        let roll: u32 = roll.into_iter().sum();
                        let mut state = state.clone();

                        let player = state.next_player();
                        let new_pos = state.move_player(player, roll);
                        state.increase_points(player, new_pos + 1);
                        state.flip_turn();

                        self.games[state.0 as usize] += universe_count;
                    }

                    self.games[game] = 0;
                }
            }
        }
    }

    pub fn win_counts(&self) -> (usize, usize) {
        let mut p1_count = 0;
        let mut p2_count = 0;
        for game in 0..self.games.len() {
            let universe_count = self.games[game];
            if universe_count != 0 {
                let state = GameState(game as u32);

                match state.get_winner() {
                    Some(Player::Player1) => p1_count += universe_count,
                    Some(Player::Player2) => p2_count += universe_count,
                    None => (),
                }
            }
        }
        (p1_count, p2_count)
    }
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let (p1, p2) = parse_positions(input)?;
    let mut game = DiracDie::new(p1, p2);
    game.play();
    let (p1_count, p2_count) = game.win_counts();
    let result = p1_count.max(p2_count);
    Ok(result as iAoc)
}
