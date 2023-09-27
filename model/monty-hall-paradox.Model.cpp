#include <iostream>
#include <vector>
#include <ctime>
#include <cstdlib> // for std::rand

class Model {
public :
    class StateResult {
    public:
        int wins;
        int losses;
        int loops;

        StateResult(int winsVal = 0, int lossesVal = 0, int loopsVal = 0)
            : wins(winsVal), losses(lossesVal), loops(loopsVal) {}
    };

    class State {
    public:
        bool host_open;
        bool player_change;
        StateResult result;

        State(bool hostOpen, bool playerChange)
            : host_open(hostOpen), player_change(playerChange), result() {}

        // Member functions to modify Result values
        void addWin() {
            result.wins++;
        }

        void addLoss() {
            result.losses++;
        }

        void addLoop() {
            result.loops++;
        }
    };

    class Session {
    public:
        std::vector<State> States;
    };

    class Curtain {
    public:
        std::string name;
        bool wins;
        bool player_chosen;
        bool host_chosen;

        Curtain(const std::string& curtainName, bool winsFlag = false)
            : name(curtainName), wins(winsFlag), player_chosen(false), host_chosen(false) {}
    };

    class Game {
    public:
        std::vector<Curtain> curtains;
        int winningCurtainIndex;

        Game() {
            for (int j = 0; j <= 2; ++j) {
                curtains.emplace_back("Curtain " + std::to_string(j));
            }
        }
    };


};
