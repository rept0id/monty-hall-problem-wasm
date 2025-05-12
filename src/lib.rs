use wasm_bindgen::prelude::*;

use rand::Rng;

use serde_wasm_bindgen::to_value;

/*** * * ***/

mod model;

/*** * * ***/

fn simulate(simulation: &mut crate::model::model::Simulation) {
    let mut rng: rand::rngs::ThreadRng;

    /*** * ***/

    rng = rand::thread_rng();

    /*** * ***/

    for s in 0..crate::model::model::CONST_STATES_SIZE as usize {
        for _ in 0..crate::model::model::CONST_STATE_GAMES_SIZE {
            // game
            let mut game: crate::model::model::Game;

            /*** * * ***/

            // game
            game = crate::model::model::Game::new();

            /*** * * ***/

            // game
            // game : do_host_reveal
            if simulation.states[s].do_host_reveal {
                // if curtains are 3, there is only one curtain to switch to,
                //  as the other is either the winning or the player choosen
                //  thus, we want a deterministic approach.
                // else, we want a purely random approach (and not one that just picks the next).
                if crate::model::model::CONST_CURTAINS_SIZE == 3 {
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

                        n = rng.gen_range(0..crate::model::model::CONST_CURTAINS_SIZE);

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
            // game : do_player_change
            if simulation.states[s].do_player_change {
                // if curtains are 3, there is only one curtain to switch to,
                //  as the other is either the winning or the host choosen
                //  thus, we want a deterministic approach.
                // else, we want a purely random approach (and not one that just picks the next).
                if crate::model::model::CONST_CURTAINS_SIZE == 3 {
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

                        n = rng.gen_range(0..crate::model::model::CONST_CURTAINS_SIZE);

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

            /*** * * ***/

            // simulation
            // simulation : player_wins_count (based on : game)
            if game.player_curtain_idx == game.win_curtain_idx {
                simulation.states[s].player_wins_count += 1;
            }
            // simulation : games
            simulation.states[s].games += 1;
        }
    }
}

/*** * * ***/

#[wasm_bindgen]
pub fn monty_hall_problem_wasm() -> wasm_bindgen::JsValue {
    // simulation
    let mut simulation: crate::model::model::Simulation;

    /*** * * ***/

    // simulation
    simulation = crate::model::model::Simulation::new();

    /*** * * ***/

    // simulation
    simulate(&mut simulation);

    /*** * * ***/

    // res (based on : simulation)
    to_value(&simulation).unwrap()
}
