use wasm_bindgen::prelude::*;

use rand::Rng;

use serde_wasm_bindgen::to_value;

/*** * * ***/

mod model;

/*** * * ***/

const STATES: i32 = 4;

const DEF_STATE_GAMES: i32 = 1_000_000;

const DEF_CURTAINS: i32 = 3;

/*** * * ***/

fn set_states_cases(simulation: &mut crate::model::model::Simulation) {
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

fn simulate(simulation: &mut crate::model::model::Simulation) {
    let mut rng = rand::thread_rng();

    for _ in 0..DEF_STATE_GAMES {
        for s in 0..STATES as usize {
            let mut game = crate::model::model::Game {
                win_curtain_idx: rng.gen_range(0..DEF_CURTAINS),
                player_curtain_idx: rng.gen_range(0..DEF_CURTAINS),
                host_curtain_idx: 0,
            };

            if simulation.states[s].do_host_reveal {
                // if curtains are 3, there is only one curtain to switch to,
                //  as the other is either the winning or the player choosen
                //  thus, we want a deterministic approach.
                // else, we want a purely random approach (and not one that just picks the next).
                if DEF_CURTAINS == 3 {
                    for i in 0..3 {
                        if i == game.win_curtain_idx {
                            continue;
                        }

                        if i == game.player_curtain_idx {
                            continue;
                        }

                        /*** * * ***/

                        game.host_curtain_idx = i;
                        break;
                    }
                } else {
                    loop {
                        let n: i32;

                        /*** * * ***/

                        n = rng.gen_range(0..DEF_CURTAINS);

                        /*** * * ***/

                        if n == game.win_curtain_idx {
                            continue;
                        }

                        if n == game.player_curtain_idx {
                            continue;
                        };

                        /*** * * ***/

                        game.host_curtain_idx = n;
                        break;
                    }
                }
            }

            if simulation.states[s].do_player_change {
                // if curtains are 3, there is only one curtain to switch to,
                //  as the other is either the winning or the host choosen
                //  thus, we want a deterministic approach.
                // else, we want a purely random approach (and not one that just picks the next).
                if DEF_CURTAINS == 3 {
                    for i in 0..3 {
                        if i == game.player_curtain_idx {
                            continue;
                        }

                        if simulation.states[s].do_host_reveal {
                            if i == game.host_curtain_idx {
                                continue;
                            }
                        }

                        /*** * * ***/

                        game.player_curtain_idx = i;
                        break;
                    }
                } else {
                    loop {
                        let n: i32;

                        /*** * * ***/

                        n = rng.gen_range(0..DEF_CURTAINS);

                        /*** * * ***/

                        if n == game.player_curtain_idx {
                            continue;
                        }

                        if simulation.states[s].do_host_reveal {
                            if n == game.host_curtain_idx {
                                continue;
                            }
                        }

                        /*** * * ***/

                        game.player_curtain_idx = n;
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

    set_states_cases(&mut simulation);
    simulate(&mut simulation);

    to_value(&simulation).unwrap()
}
