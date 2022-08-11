use std::{collections::HashMap};
use lazy_static::*;
use std::sync::Mutex;
use anyhow::Result;
use wasmtime::*;

wit_bindgen_wasmtime::export!("imports.wit");
wit_bindgen_wasmtime::import!("exports.wit");

use imports::*;
use exports::*;

// 供modules调用的函数
fn f1(s: &String) -> &String {
    println!("from f1, {}", s);
    s
}


fn f2(s: &String) -> &String {
    println!("from f2, {}", s);
    s
}

#[derive(Default)]
pub struct MyImports;

impl Imports for MyImports {
    // 暴露给wasm module的函数

    fn proxy(&mut self, name: &str, param: &str) -> String {
        let mut map = HASHMAP.lock().unwrap();
        let param = String::from(param);

        let rs = map.get(name).unwrap()(&param);
        "sd".into()
    }

}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, fn(&String)->&String>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };

    static ref MODULE_FUNC: Mutex<HashMap<String, (Exports<Context<MyImports, ExportsData>>, Store<Context<MyImports, ExportsData>>)>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

struct Context<I, E> {
    wasi: wasmtime_wasi::WasiCtx,
    imports: I,
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

fn instantiate<I: Default, E: Default, T>(
    wasm: &str,
    add_imports: impl FnOnce(&mut Linker<Context<I, E>>) -> Result<()>,
    mk_exports: impl FnOnce(
        &mut Store<Context<I, E>>,
        &Module,
        &mut Linker<Context<I, E>>,
    ) -> Result<(T, Instance)>,
) -> Result<(T, Store<Context<I, E>>)> {
    let engine = Engine::new(&default_config()?)?;
    let module = Module::from_file(&engine, wasm)?;

    let mut linker = Linker::new(&engine);
    add_imports(&mut linker)?; //装载我们的实现到linker
    wasmtime_wasi::add_to_linker(&mut linker, |cx| &mut cx.wasi)?;


    let mut store = Store::new(
        &engine,
        Context {
            wasi: default_wasi(),
            imports: I::default(),
            exports: E::default(),
        },
    );
    let (exports, _instance) = mk_exports(&mut store, &module, &mut linker)?;

    // for (key, value, _) in linker.iter(&mut store) {
    //     println!("{} / {}", key, value);

    // }

    Ok((exports, store))
}

fn registry(name: &str, f: fn(&String)->&String) {
    {
        let mut map = HASHMAP.lock().unwrap();
        map.insert(String::from(name), f);
    }
}

fn registry_module(path: &str, name: &str) -> Result<()> {
    let (e, mut s) = instantiate(
        path,
        |linker| imports::add_to_linker(linker, |cx| -> &mut MyImports { &mut cx.imports }),
        |store, module, linker| Exports::instantiate(store, module, linker, |cx| &mut cx.exports),
    )?;
    {
        let mut map = MODULE_FUNC.lock().unwrap();
        map.insert(String::from(name), (e, s));
    }
    Ok(())
}

fn call_module_func(mname: &str, fname: &str, param: &str) -> String {

    let mut map = MODULE_FUNC.lock().unwrap();
    let (e, mut s) = map.remove(mname).unwrap();
    let rs = e.proxy(&mut s, fname, param);      
    map.insert(String::from(mname), (e, s));

    rs.unwrap()
}

pub fn load_module_by_name(module_name: String) -> Result<()> {
    let res = registry_module(&(module_name.clone() + ".wasm"), &module_name);
    match res {
        Ok(_) => {
            println!("load module {} success", module_name);
            // call_module_func(&module_name, "modulef1", "call after first load");
        }
        Err(e) => {
            println!("load module {} failed, {}", module_name, e);
        }
    }
    
    Ok(())
}

pub fn unload_module_by_name(module_name: String) -> Result<()> {
    let mut map = MODULE_FUNC.lock().unwrap();
    match map.remove(&module_name) {
        Some(_) => {
            println!("unload module {} success", module_name);
        }
        None => {
            println!("unload module {} failed, module has not been loaded", module_name);
        }
    }
    Ok(())
}

pub fn show_module_list() -> Result<()> {
    let map = MODULE_FUNC.lock().unwrap();
    for (key, _) in map.iter() {
        println!("{}", key);
    }
    Ok(())
}

pub fn initialize() -> Result<()> {
    registry("f1", f1);
    registry("f2", f2);
    Ok(())
}
