mod cli;
use cli::{Action::*, CommandLineArgs};
use core::time;
use std::io;
use structopt::StructOpt;

mod load_module;
use load_module::*;
use std::thread;

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
                    load_module_by_path(&path, &name).unwrap();

                }
                Unload { module_name } => {
                    println!("Unloading module {}", module_name);
                    unload_module_by_name(module_name).unwrap();

                }
                List => {
                    println!("Loaded modules list:");
                    show_module_list().unwrap();
                    // server_test();
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
                    let param:serde_json::Value = serde_json::from_str(&param.as_str()).unwrap();
                    call_module_func(&module_name, &function_name, &param).unwrap();
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

fn test() {
    loop {
        let dur = time::Duration::from_millis(3000);
        thread::sleep(dur);
        
        let param:serde_json::Value = serde_json::from_str("{\"message\":\"hello_from_cli\"}").unwrap();
        println!("call from test thread");
        call_module_func("A", "modulef1", &param);
    }
}

fn main() {
    load_module::initialize().unwrap();
    let mut children = vec![];

    children.push(
        thread::spawn(move || {
            cli();
        }));

        children.push(
            thread::spawn(move || {
                http_server::start();
            }));
    
    // children.push(
    //     thread::spawn(move || {
    //         test();
    //     }));

    for child in children {
        let _ = child.join();
    }
}
