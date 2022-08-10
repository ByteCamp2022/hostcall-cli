use anyhow::Result;
use wasmtime::*;

wit_bindgen_wasmtime::import!("wasm/wits/module1.wit");

struct Context<E> {
    wasi: wasmtime_wasi::WasiCtx,
    exports: E,
}

fn default_config() -> Result<Config> {
    // Create an engine with caching enabled to assist with iteration in this
    // project.
    let mut config = Config::new();
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    Ok(config)
}

fn default_wasi() -> wasmtime_wasi::WasiCtx {
    wasmtime_wasi::sync::WasiCtxBuilder::new()
        .inherit_stdio()
        .build()
}

fn instantiate<E: Default, T>(
    wasm: &str,
    mk_exports: impl FnOnce(
        &mut Store<Context<E>>,
        &Module,
        &mut Linker<Context<E>>,
    ) -> Result<(T, Instance)>,
) -> Result<(T, Store<Context<E>>)> {
    println!("instantiate");
    let engine = Engine::new(&default_config()?)?;
    let module = Module::from_file(&engine, wasm)?;

    let mut linker = Linker::new(&engine);
    println!("initialized linker");
    // wasmtime_wasi::add_to_linker(&mut linker, |s| &s.wasi)?;
    wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context<E>| &mut cx.wasi)?;

    let mut store = Store::new(
        &engine,
        Context {
            wasi: default_wasi(),
            exports: E::default(),
        },
    );
    println!("initialized store");
    let res = mk_exports(&mut store, &module, &mut linker);
    let exports: T;
    let _instance: Instance;
    match res {
        Ok((exp, _ins)) => {
            println!("got exports");
            exports = exp;
            _instance = _ins;
        }
        Err(err) => {
            println!("error: {}", err);
            return Err(err);
        }
    }
    Ok((exports, store))
}

pub fn load_module_by_path(path: String) -> Result<()> {
    let (exports, mut store) = instantiate(
        "module1.wasm",
        |store, module, linker| module1::Module1::instantiate(store, module, linker, |cx| &mut cx.exports),
    )?;
    println!("calling f1");
    exports.f1(&mut store, "sdf")?;
    println!("calling f2 and found reult: {}", exports.f2(&mut store)?);
    
    Ok(())
}

// Problem:
// mod A {
//     fn f1(i: i32) -> i32 {
//         // ...
//         i * 2
//     }
//     fn f2(i: i32) -> i32 {
//         // ...
//         i + 233
//     }
// }

// mod B {
//     fn f1(i: i32) -> i32 {
//         // ...
//         i
//     }
// }

// mod C {
//     fn f1(i: i32) -> i32 {
//         // ...
//         i / 2
//     }
// }

// // ....

// fn call(mod_name: String, func_name: String, args: Vec<i32>) -> Result<i32> {
//     let module = Module::new(&mod_name)?;
//     let func = module.get_func(&func_name)?;
//     let ret = func.call(&args)?;
//     Ok(ret.get_i32()?)
// }