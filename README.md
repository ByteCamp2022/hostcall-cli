# A Cli Example for Rust and WebAssembly Hostcall

## Usage

```bash
# build wasms for ervery module in the `wasm` directory and move them to the root directory
# example:
cd wasm/module_A
cargo build --target wasm32-wasi --release

cp target/wasm32-wasi/release/module_A.wasm ../../

# build and run the Cli
cd ../../src
cargo run
```

## Add new wasm modules

Just follow what was done in module_A.
