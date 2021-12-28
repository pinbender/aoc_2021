use std::collections::HashMap;

pub fn solve(lines: &[&str]) -> (usize, u64) {
    let (player_1, player_2) = parse(lines);

    let pt_1 = solve_1(player_1, player_2);

    let pt_2 = solve_2(player_1, player_2);

    (pt_1,pt_2)
}

fn parse(lines: &[&str]) -> (u8, u8) {
    (lines[0].parse::<u8>().unwrap() - 1,
    lines[1].parse::<u8>().unwrap() - 1)
}

fn solve_1(player_1: u8, player_2: u8) -> usize {
    let mut players = vec![Player::new(player_1), Player::new(player_2)];
    let mut p = 0;
    let mut dice = Dice::default();
    loop {
        let roll = (0..3).map(|_| dice.roll() as u32).sum();
        players[p].turn(roll);
        if players[p].score >= 1000 {
            break;
        }
        p = (p + 1) % players.len();
    }

    let loser_score = players.iter().map(|p| p.score).min().unwrap();

    (loser_score * dice.num_rolls) as usize
}

type MultiPlayer = HashMap::<((u8,u8), (u8, u8)), u64>;
fn solve_2(player_1: u8, player_2: u8) -> u64 {
    let mut states = MultiPlayer::default();
    states.insert(((player_1, 0), (player_2, 0)), 1);

    let mut winners = [0 as u64; 2];

    let mut p = false;
    while !states.is_empty() {
        let (new_states, new_winners) = 
            turn_2(&states, p);
        states = new_states;
        winners[p as usize] += new_winners;
        p = !p;
    }

    winners.into_iter().max().unwrap()
}

fn turn_2(mp: &MultiPlayer, player: bool) -> (MultiPlayer, u64) {
    mp.iter().fold((MultiPlayer::default(), 0), 
        |mut result, (board_state, board_count)| {
            let (multi, winners) = turn_2_multi(board_state, player);
            for (multi_state, multi_count) in multi.iter() {
                *result.0.entry(*multi_state).or_insert(0) += multi_count * board_count;
            }
            (result.0, result.1 + winners * board_count)
        })
}

fn get_turn_2_weight(n: u8) -> u64 {
    match n {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => unreachable!("Invalid dice roll")
    }
}

fn turn_2_multi(state: &((u8, u8), (u8, u8)), player: bool) -> (MultiPlayer, u64) {
    (3..=9).fold((MultiPlayer::default(), 0), |mut result, n| {
        let weight = get_turn_2_weight(n);
        let player_state = if player { state.1 } else { state.0 };
        if let Some(single_state) = turn_2_single(&player_state, n) {
            let new_state = 
                if player { (state.0, single_state) } 
                else { (single_state, state.1) };
            *result.0.entry(new_state).or_insert(0) += weight;
        } else {
            result.1 += weight;
        }
        result
    })
}

fn turn_2_single(state: &(u8, u8), dice: u8) -> Option<(u8, u8)> {
    let new_pos = (state.0 + dice) % 10;
    let new_score = state.1 + new_pos + 1;
    (new_score < 21).then(|| (new_pos, new_score))
}

// Part 1 hyper-engineering in case part 2 was part 1 with weird dice or something.

#[derive(Debug)]
struct Player {
    score: u32,
    position: u8
}

impl Player {
    fn new(position: u8) -> Self {
        Self {
            score: 0,
            position
        }
    }

    fn turn(&mut self, distance: u32) {
        self.position = ((self.position as u32 + distance) % 10) as u8;
        self.score += self.position as u32 + 1;
    }
}

#[derive(Default)]
struct Dice {
    num_rolls: u32,
}

impl Dice {
    fn roll(&mut self) -> u8 {
        let result = self.num_rolls % 100 + 1;
        self.num_rolls += 1;
        result as u8
    }
}