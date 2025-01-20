prepare:
	rustup target add wasm32-unknown-unknown;
	cargo install wasm-bindgen-cli;

build:
	cargo build --release --target wasm32-unknown-unknown;
	wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/release/monty_hall_problem_wasm.wasm;
