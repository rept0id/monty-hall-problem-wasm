#include <iostream>
#include <vector>
#include <ctime>
#include "model/monty-hall-paradox.Model.cpp" // Include your model here
#include <cstdlib> // for std::rand

int main() {
    // Seed the random number generator with the current time
    std::srand(static_cast<unsigned int>(std::time(nullptr)));

    // Define the number of simulations
    int numSimulations = 1000000;

    // Create an instance of your Model
    Model model;
    Model::Session session;

    // Get the list of states from your model
    session.States.emplace_back(true, true);
    session.States.emplace_back(true, false);
    session.States.emplace_back(false, true);
    session.States.emplace_back(false, false);

    // Simulate for each state and accumulate results
    for (int i = 0; i < numSimulations; ++i) {
        for (Model::State& state : session.States) {
            Model::Game game;

            game.curtains[(std::rand() % 3)].wins = true;

            // Make one of the curtains chosen by the player (1, 2, or 3)
            int chosenCurtainIndex = std::rand() % 3;
            game.curtains[chosenCurtainIndex].player_chosen = true;

            // Handle host choosing if "host_open" is true
            if (state.host_open) {
                int hostChosenIndex;
                do {
                    hostChosenIndex = std::rand() % 3;
                } while (game.curtains[hostChosenIndex].wins || game.curtains[hostChosenIndex].player_chosen);
                game.curtains[hostChosenIndex].host_chosen = true;
            }

            // Handle player changing if "player_change" is true
            if (state.player_change) {
                int newChosenIndex;
                do {
                    newChosenIndex = std::rand() % 3;
                } while (game.curtains[newChosenIndex].player_chosen || game.curtains[newChosenIndex].host_chosen);

                game.curtains[chosenCurtainIndex].player_chosen = false;
                game.curtains[newChosenIndex].player_chosen = true;

                chosenCurtainIndex = newChosenIndex;
            }

            // Check if the chosen curtain wins
            if (game.curtains[chosenCurtainIndex].wins) {
                state.addWin();
            } else {
                state.addLoss();
            }

            // Always call addLoop
            state.addLoop();
        }
    }

    // Print the final results for all states after all simulations
    for (const Model::State& state : session.States) {
        std::cout << "For state (host_open=" << state.host_open << ", player_change=" << state.player_change << "):\n";
        std::cout << "Wins: " << state.result.wins << ", Losses: " << state.result.losses << ", Loops: " << state.result.loops << std::endl;
    }

    return 0;
}
