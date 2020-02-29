build:
	cargo build
	wasm-pack build

web-example: build
	pushd examples/web; npm start
