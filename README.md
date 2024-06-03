# monty-hall-paradox

![screenshot](https://github.com/rept0id/monty-hall-paradox/blob/main/screenshot.png)

Demo : [https://monty-hall-paradox.simplecode.gr/](https://monty-hall-paradox.simplecode.gr/)

Imagine you're on a game show with three doors. Behind one is a prize, while the other two hide goats. You pick a door, hoping for the prize. Then, the host, who knows what's behind each door, opens one of the other doors to reveal a goat. Now, you're asked: do you stick with your first choice or switch to the remaining closed door? Surprisingly, statistics show you're better off switching. It's a tricky probability puzzle that shows how our instincts can sometimes lead us astray.

This software runs simulations of the Monty Hall Paradox numerous times. In some iterations, the player opts to change doors at the end, while in others, they stick with their initial choice. Through these simulations, it's proven that statistics hold true. Even after running the simulation millions of times, it consistently demonstrates that switching doors yields the best winning probabilities.

## How to run
If you dont have emcc (emscripten compiler) install it via :
```
sudo apt-get install emscripten
```

Then compile and build :
```
emcc monty-hall-paradox.cpp -o index.html -s WASM=1
```

Then simply use a server to view the builded index.html from a browser. For example node's http-server :
```
http-server
```
( If you dont have node server : `sudo apt install nodejs npm`, `npm i http-server`, then again `http-server`)

---

Thanks to Irina Kalman !
