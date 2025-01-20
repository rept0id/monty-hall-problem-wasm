use wasm_bindgen::prelude::*;

use rand::Rng;

use serde_wasm_bindgen::to_value;

/*** * * ***/

pub mod model;

/*** * * ***/

const STATES: i32 = 4;

const DEF_STATE_GAMES: i32 = 1_000_000;

const DEF_CURTAINS: i32 = 3;

/*** * * ***/

fn init_states_simulation(simulation: &mut crate::model::model::Simulation) {
    // Iterate over the states vector by index
    for (i, state) in simulation.states.iter_mut().enumerate() {
        match i {
            0 => {
                state.do_host_reveal = true;
                // state.do_player_change = false;
            }
            1 => {
                state.do_host_reveal = true;
                state.do_player_change = true;
            }
            // 2 => {
            //     // state.do_host_reveal = false;
            //     // state.do_player_change = false;
            // }
            3 => {
                // state.do_host_reveal = false;
                state.do_player_change = true;
            }
            _ => {}
        }
    }
}

pub fn simulate(simulation: &mut crate::model::model::Simulation) {
    let mut rng = rand::thread_rng();

    for _ in 0..DEF_STATE_GAMES {
        for s in 0..STATES as usize {
            let mut game = crate::model::model::Game {
                win_curtain_idx: rng.gen_range(0..DEF_CURTAINS),
                player_curtain_idx: rng.gen_range(0..DEF_CURTAINS),
                host_curtain_idx: 0,
            };

            if simulation.states[s].do_host_reveal {
                loop {
                    game.host_curtain_idx = rng.gen_range(0..DEF_CURTAINS);
                    if game.host_curtain_idx != game.win_curtain_idx {
                        break;
                    }
                }
            }

            if simulation.states[s].do_player_change {
                loop {
                    let new_player_curtain_idx = rng.gen_range(0..DEF_CURTAINS);
                    if new_player_curtain_idx != game.player_curtain_idx
                        && new_player_curtain_idx != game.host_curtain_idx
                    {
                        game.player_curtain_idx = new_player_curtain_idx;
                        break;
                    }
                }
            }

            if game.player_curtain_idx == game.win_curtain_idx {
                simulation.states[s].player_wins_count += 1;
            }
        }
    }
}

/*** * * ***/

#[wasm_bindgen]
pub fn monty_hall_problem_wasm() -> wasm_bindgen::JsValue {
    let mut simulation = crate::model::model::Simulation::new(STATES as usize);

    init_states_simulation(&mut simulation);
    simulate(&mut simulation);

    to_value(&simulation).unwrap()
}
