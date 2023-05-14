let win = false;

let prizes = [
  { name: "car", win: true },
  { name: "clown", win: false },
  { name: "goat", win: false },
];

let curtains = [
  { name: "curtain1", prize: "", chosen: false, chosenHost: false },
  { name: "curtain2", prize: "", chosen: false, chosenHost: false },
  { name: "curtain3", prize: "", chosen: false, chosenHost: false },
];

async function main() {
  let wins = {};
  wins.SwapFalse = 0;
  wins.SwapTrue = 0;

  for (let i = 0; i < 1000 + 1; i++) {
    if (game(false) === true) {
      wins.SwapFalse++;
    }
    if (game(true) === true) {
      wins.SwapTrue++;
    }
  }

  console.log(wins.SwapFalse);
  console.log(wins.SwapTrue);
}

function game(swap) {
  newGame();

  chooseCurtain();
  hostChoice();
  playerSwap(swap);
  checkWin();

  return win;
}

function checkWin() {
  curtains.forEach((curtain, i) => {
    if (curtain.chosen && curtain.prize.win) {
      win = true;
    }
  });
}

function playerSwap(swap) {
  if (swap) {
    curtains.forEach((curtain, i) => {
      if (!curtain.chosen && !curtain.chosenHost) {
        curtains.forEach((curtain2) => {
          curtain2.chosen = false;
        });
        curtain.chosen = true;
      }
    });
  }
}

function chooseCurtain() {
  let randomChoice = Math.floor(Math.random() * prizes.length);
  curtains[randomChoice].chosen = true;
}

function hostChoice() {
  let hostOptions = [];

  curtains.forEach((curtain, i) => {
    if (!curtain.chosen && curtain.prize.win === false) {
      hostOptions.push(i);
    }
  });

  let randomChoice =
    hostOptions[Math.floor(Math.random() * hostOptions.length)];
  curtains[randomChoice].chosenHost = true;
}

function newGame() {
  win = false;

  let randomPrizesOrder = prizes.sort(() => Math.random() - 0.5);
  curtains.forEach((curtain, i) => {
    curtain.chosen = false;
    curtain.chosenHost = false;
    curtain.prize = randomPrizesOrder[i];
  });
}

main();
