use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Game {
    pub win_curtain_idx: i32,
    pub player_curtain_idx: i32,
    pub host_curtain_idx: i32,
}

#[derive(Debug, Serialize)]
pub struct State {
    pub do_player_change: bool,
    pub do_host_reveal: bool,
    pub player_wins_count: i32,
}

#[derive(Debug, Serialize)]
pub struct Simulation {
    pub states: Vec<State>, // Use a `Vec` for a dynamically sized collection of states
}
