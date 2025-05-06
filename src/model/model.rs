use serde::Serialize;

use rand::Rng;

/*** * * ***/

pub const CONST_STATES_MAX_IDX: i32 = 3;
pub const CONST_STATE_GAMES_MAX_IDX: i32 = 999999;
pub const CONST_CURTAINS_MAX_IDX: i32 = 2;

/*** * * ***/

#[derive(Debug, Serialize)]
pub struct Game {
    pub win_curtain_idx: i32,
    pub player_curtain_idx: i32,
    pub host_curtain_idx: i32,
}

impl Game {
    pub fn new() -> Self {
        let mut rng: rand::rngs::ThreadRng;

        /*** * ***/

        rng = rand::thread_rng();

        /*** * ***/

        Self {
            win_curtain_idx: rng.gen_range(0..(crate::model::model::CONST_CURTAINS_MAX_IDX + 1)),
            player_curtain_idx: rng.gen_range(0..(crate::model::model::CONST_CURTAINS_MAX_IDX + 1)),
            host_curtain_idx: rng.gen_range(0..(crate::model::model::CONST_CURTAINS_MAX_IDX + 1)),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct State {
    pub do_player_change: bool,
    pub do_host_reveal: bool,
    pub player_wins_count: i32,
    pub games: i32,
}

impl State {
    pub fn new(do_player_change: bool, do_host_reveal: bool) -> Self {
        Self {
            do_player_change,
            do_host_reveal,
            player_wins_count: 0,
            games: 0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Simulation {
    pub states: [State; (CONST_STATES_MAX_IDX + 1) as usize],
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            states: [
                State::new(
                    false, // do_player_change
                    false, // do_host_reveal
                ),
                State::new(
                    true,  // do_player_change
                    false, // do_host_reveal
                ),
                State::new(
                    false, // do_player_change
                    true,  // do_host_reveal
                ),
                State::new(
                    true, // do_player_change
                    true, // do_host_reveal
                ),
            ],
        }
    }
}
