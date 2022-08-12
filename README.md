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


## demostrate how hot-update works

```bash
load module_A.wasm A
```

```
output:
    Loading module from module_A.wasm
    load module module_A.wasm success, and registry as A
```

```bash
call A modulef1 {"message":"hello_from_cli"}
```

```
output:
    Calling function modulef1 in module A with param {"message":"hello_from_cli"}
    enter module a, message: "hello_from_cli"
    enter host f1, message: "implemented in host"
```

```bash
load module_B.wasm A
```

```
output:
    Loading module from module_B.wasm
    load module module_B.wasm success, and registry as A
```

```bash
call A modulef1 {"message":"hello_from_cli"}
```

```
output:
    Calling function modulef1 in module A with param {"message":"hello_from_cli"}
    enter module b, message: "hello_from_cli"
    enter host f1, message: "implemented in host"
```

with test
```
output:
    call from test thread
    module or function not found
    load module_B.wasm A
    Loading module from module_B.wasm
    module or function not found
    load module module_B.wasm success, and registry as A
    enter module b, message: "hello_from_cli"
    enter host f1, message: "implemented in host"

    enter module b, message: "hello_from_cli"
    enter host f1, message: "implemented in host"

    enter module b, message: "hello_from_cli"
    enter host f1, message: "implemented in host"
```
