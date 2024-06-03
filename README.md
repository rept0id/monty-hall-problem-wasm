# monty-hall-paradox

![screenshot](https://github.com/rept0id/monty-hall-paradox/blob/main/screenshot.png)

Demo : [https://monty-hall-paradox.simplecode.gr/](https://monty-hall-paradox.simplecode.gr/)

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
