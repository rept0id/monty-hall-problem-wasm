# monty-hall-paradox

![screenshot](https://github.com/rept0id/monty-hall-paradox/blob/main/screenshot.png)

## How to run
If you dont have emcc (emscripten compiler) install it via :
```
sudo apt-get install emscripten
```

Then run, to compile :
```
emcc monty-hall-paradox.cpp -o index.html -s WASM=1
```

Then its best to run it via node server
```
http-server
```

If you dont have node server, install node and npm and then http-server

```
sudo apt install nodejs npm
npm i http-server
```

It will provide you with a link where you can see monty-hall-paradox simulation running... Have fun !

![screenshot-http-server](https://github.com/rept0id/monty-hall-paradox/blob/main/screenshot2.png)
