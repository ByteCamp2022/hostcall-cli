use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;
use anyhow::Result;
use serde_json::json;

use crate::load_module::*;

fn streram_handle(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // to check the request contents
    // let foo: Box<[u8]> = Box::new(buffer);
    // let bar = foo.into_vec();
    // println!("\n\n{}\n\n", String::from_utf8(bar).unwrap());

    let sucess = b"GET / HTTP/1.1\r\n";
    let home = b"GET /home HTTP/1.1\r\n";

    let resp = if buffer.starts_with(sucess) {
        println!("module_200 loaded.");
        call_module_func("module_200", "response", &json!({"path": "hello.html",})).unwrap()
    } else if buffer.starts_with(home) {
        println!("module_home loaded.");
        
        call_module_func("module_home", "response", &json!({"path": "home.html",})).unwrap()
    } else {
        println!("module_404 loaded.");
        call_module_func("module_404", "response", &json!({"path": "404.html",})).unwrap()
    };

    stream.write(resp["response"].as_str().unwrap().as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn start() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    load_module_by_path(&"module_200.wasm".to_string(), &"module_200".to_string());
    load_module_by_path(&"module_home.wasm".to_string(), &"module_home".to_string());
    load_module_by_path(&"module_404.wasm".to_string(), &"module_404".to_string());

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        streram_handle(&mut stream);
    }

    Ok(())
}


pub fn server_test() {
    show_module_list().unwrap();
}