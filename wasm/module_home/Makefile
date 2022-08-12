WASMLINK      ?= wasmlink
WASMTIME      ?= wasmtime
CARGO         ?= cargo

build:
	$(CARGO) build --target wasm32-wasi --release
	cp target/wasm32-wasi/release/module_home.wasm ../../


