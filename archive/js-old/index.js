function main() {
  let options = {
    loops : Math.pow(10,6)
  }

  let expirementsHostCases = [
    {"Name" : "hostTrue", "Result" : {}, "host": true},
    {"Name" : "hostFalse", "Result" : {}, "host" : false}
  ];

  expirementsHostWrapCases = expirementsHostCasesWrap(expirementsHostCases, options);

  console.log(JSON.stringify(expirementsHostWrapCases));
}

function expirementsHostCasesWrap(expirementsHostCases, options) {
  let expirements;

  expirementsHostCases.forEach((expirementsHostCase, i)=>{
    expirements = expirementsInit();
    for (let i = 0; i < options.loops; i++) {
      expirements = game(expirements, false, expirementsHostCase.host);
      expirements = game(expirements, true, expirementsHostCase.host);
    }
    expirementsHostCase.Result = expirements;
  });

  return expirementsHostCases;
}

function expirementsInit(expirements) {
  expirements = {
    curtains : [
      { name: "curtain1", prize: "", chosen: false, chosenHost: false },
      { name: "curtain2", prize: "", chosen: false, chosenHost: false },
      { name: "curtain3", prize: "", chosen: false, chosenHost: false },
    ],
    prizes : [
      { name: "car", win: true },
      { name: "clown", win: false },
      { name: "goat", win: false },
    ],
    cases : {
      swapTrue : {
        wins : 0,
        losses : 0,
        loops : 0
      },
      swapFalse : {
        wins : 0,
        losses : 0,
        loops : 0
      }
    },
    expirementTemp : { }
  };
  
  return expirements;
}

function game(expirements, swap, allowHost = true) {
  newGame(expirements);

  chooseCurtain(expirements);
  if (allowHost) {
    hostChoice(expirements, swap);
  }
  playerSwap(swap, expirements);
  checkWin(expirements, swap);

  return expirements;
}

function checkWin(expirements, swap) {
  expirements.curtains.forEach((curtain, i) => {
    if (curtain.chosen && curtain.prize.win) {
      if (swap) {
        expirements.cases.swapTrue.wins++;
        expirements.expirementTemp.win = true;
      } else {
        expirements.cases.swapFalse.wins++;
        expirements.expirementTemp.win = true;
      }
    }
  });

  if (!expirements.expirementTemp.win) {
    if (swap) {
      expirements.cases.swapTrue.losses++;
    } else {
      expirements.cases.swapFalse.losses++;
    }
  }

  if (swap) {
    expirements.cases.swapTrue.loops++;
  } else {
    expirements.cases.swapFalse.loops++;
  }
}

function playerSwap(swap, expirements) {
  if (swap) {
    expirements.curtains.forEach((curtain, i) => {
      if (!curtain.chosen && !curtain.chosenHost) {
        expirements.curtains.forEach((curtain2) => {
          curtain2.chosen = false;
        });
        curtain.chosen = true;
      }
    });
  }
}

function chooseCurtain(expirements) {
  let randomChoice = Math.floor(Math.random() * expirements.prizes.length);
  expirements.curtains[randomChoice].chosen = true;
}

function hostChoice(expirements) {
  let hostOptions = [];

  expirements.curtains.forEach((curtain, i) => {
    if (!curtain.chosen && curtain.prize.win === false) {
      hostOptions.push(i);
    }
  });

  let randomChoice =
    hostOptions[Math.floor(Math.random() * hostOptions.length)];
    expirements.curtains[randomChoice].chosenHost = true;
}

function newGame(expirements) {
  expirements.expirementTemp.win = false;

  let randomPrizesOrder = expirements.prizes.sort(() => Math.random() - 0.5);
  expirements.curtains.forEach((curtain, i) => {
    curtain.chosen = false;
    curtain.chosenHost = false;
    curtain.prize = randomPrizesOrder[i];
  });


}

main();
