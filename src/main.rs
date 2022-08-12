mod cli;
use cli::{Action::*, CommandLineArgs};
use std::io;
use structopt::StructOpt;

mod load_module;
use load_module::*;
use std::thread;
use std::net::TcpStream;
use std::io::prelude::*;

mod http_server;


fn cli() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut args: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        args.insert(0, String::new());
        let args = CommandLineArgs::from_iter_safe(&args);
        match args {
            Ok(args) => match args.action {
                Load { path, name } => {
                    println!("Loading module from {}", path);
                    load_module_by_path(&path, &name);
                }
                Unload { module_name } => {
                    println!("Unloading module {}", module_name);
                    unload_module_by_name(module_name);
                }
                List => {
                    println!("Loaded modules list:");
                    show_module_list();
                }
                Call {
                    module_name,
                    function_name,
                    param,
                } => {
                    println!(
                        "Calling function {} in module {} with param {}",
                        function_name, module_name, param
                    );
                    let param: serde_json::Value = serde_json::from_str(&param.as_str()).unwrap();
                    call_module_func(&module_name, &function_name, &param);
                }
                Request {
                    url, 
                    port,
                    path,
                } => {
                    println!(
                        "\nSending request to {}:{}{}",
                        url, port, path
                    );
                    let address = format!(
                        "{}:{}",
                        url, port
                    );
                    let mess = format!(
                        "GET {} HTTP/1.1\r\n",
                        path
                    );
                    let mut stream = TcpStream::connect(address).unwrap();
                    stream.write(mess.as_bytes()).expect("request failed");
                    println!("Sending done\n");
                }
                Exit => {
                    println!("Exiting");
                    break;
                }
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

fn main() {
    load_module::initialize().unwrap();
    let mut children = vec![];

    children.push(thread::spawn(move || {
        cli();
    }));

    children.push(thread::spawn(move || {
        http_server::start();
    }));

    for child in children {
        let _ = child.join();
    }
}

#[test]
fn test() {
    use std::time::Duration;

    load_module::initialize().unwrap();
    let mut children = vec![];

    children.push(thread::spawn(move || {
        cli();
    }));

    // children.push(thread::spawn(move || {
    //     load_module_by_path(&String::from("module_A.wasm"), &String::from("A")).unwrap();
    //     loop {
    //         thread::sleep(Duration::from_millis(3000));

    //         let param: serde_json::Value =
    //             serde_json::from_str("{\"message\":\"hello_from_cli\"}").unwrap();
    //         println!("call from test thread");
    //         call_module_func("A", "modulef1", &param);
    //     }
    // }));

    for child in children {
        let _ = child.join();
    }
}
