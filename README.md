# monty-hall-paradox

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
