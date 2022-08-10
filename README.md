# A Cli Example for Rust and WebAssembly Hostcall

## Usage

```bash
# build wasms for ervery module in the `wasm` directory and move them to the root directory
# example:
cd wasm/module1
cargo build --target wasm32-wasi --release

cp target/wasm32-wasi/release/module1.wasm ../../

# build and run the Cli
cd ../../src
cargo run
```

## Add new wasm modules

Just follow what was done in module1.

Write a wit file in the `wits` directory, and create a new cargo project in the `wasm` directory.

Remember to keep **All** the module name the same (for example `module1` and `Module1` below).

```Rust
wit_bindgen_rust::export!("../wits/module1.wit");

struct Module1;

impl module1::Module1 for Module1 {
    // ...
}
```