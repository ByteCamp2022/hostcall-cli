use std::{collections::HashMap};
use lazy_static::*;
use std::sync::Mutex;
use anyhow::Result;
use wasmtime::*;
use serde_json::json;
use std::fs;

// use std::thread;
// use std::time::Duration;

wit_bindgen_wasmtime::export!("imports.wit");
wit_bindgen_wasmtime::import!("exports.wit");

use imports::*;
use exports::*;

// host侧 供modules调用的函数
fn f1(v: &serde_json::Value) -> serde_json::Value {
    println!("enter host f1, message: {}", v["message"]);
    println!("");
    let rs = json!({
        "message": "ok",
      });
    rs
}

fn f2(v: &serde_json::Value) -> serde_json::Value {
    println!("enter host f2, message: {}", v["message"]);
    println!("");
    let rs = json!({
        "message": "ok",
      });
    rs
}

fn f3(v: &serde_json::Value) -> serde_json::Value {
    println!("f3");
    println!("");
    serde_json::from_str("{}").unwrap()
}

fn responseStatus(v: &serde_json::Value) -> serde_json::Value {
    let rs = json!({
        "status": v["status"].as_str().unwrap(),
      });
    rs
}

fn response_HTML(v: &serde_json::Value) -> serde_json::Value {
    let html = fs::read_to_string(v["path"].as_str().unwrap()).unwrap();
    // return html;
    let rs = json!({
        "html": html,
      });
    rs
}

fn response(v: &serde_json::Value) -> serde_json::Value {
    let rs = json!({
        "response": v["response"].as_str().unwrap(),
      });
    rs
}

// fn response_HTML(path: String) -> String {
//     let html = fs::read_to_string(path).unwrap();
//     return html;
// }

// fn response(path: String) -> String {
//     let s = String::from(" ");
//     let status = responseStatus(s);
//     let contents = response_HTML(path);
//     let resp = format!(
//         "{}\r\nContent-Length: {}\r\n\r\n{}",
//         status,
//         contents.len(),
//         contents
//     );
//     return resp;
// }
// -----------------------------------------

#[derive(Default)]
pub struct MyImports;

impl Imports for MyImports {
    // 暴露给wasm module的函数

    fn proxy(&mut self, name: &str, param: &str) -> String {
        let mut map = HASHMAP.lock().unwrap();
        let param = String::from(param);
        let v:serde_json::Value = serde_json::from_str(&param).unwrap();
        let rs = map.get(name).unwrap()(&v);
        rs.to_string()
    }
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, fn(&serde_json::Value)->serde_json::Value>> = {
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

    Ok((exports, store))
}

fn registry(name: &str, f: fn(&serde_json::Value)->serde_json::Value) {
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

pub fn call_module_func(mname: &str, fname: &str, param: &serde_json::Value) -> Result<serde_json::Value> {

    let mut map = MODULE_FUNC.lock().unwrap();
    let e: Exports<Context<MyImports, ExportsData>>;
    let mut s: Store<Context<MyImports, ExportsData>>;
    if let Some((exports, store)) = map.remove(mname) {
        e = exports;
        s = store;
    } else {
        println!("module or function not found");
        return Err(anyhow::anyhow!("module or function not found"));
    }
    let rs = e.proxy(&mut s, fname, &param.to_string());      
    map.insert(String::from(mname), (e, s));
    let v:serde_json::Value = serde_json::from_str(rs.unwrap().as_str()).unwrap();
    Ok(v)
}


pub fn load_module_by_path(path :&String, name :&String) -> Result<()> {
    // thread::sleep(Duration::from_millis(2000));
    let res = registry_module(&path, &name);
    match res {
        Ok(_) => {
            println!("load module {} success, and registry as {}", path, name);
        }
        Err(e) => {
            println!("load module {} failed, {}", path, e);
        }
    }
    
    Ok(())
}

pub fn unload_module_by_name(module_name: String) -> Result<()> {
    // thread::sleep(Duration::from_millis(2000));
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

    registry("responseStatus", responseStatus);
    registry("response_HTML", response_HTML);
    registry("response", response);
    
    Ok(())
}
