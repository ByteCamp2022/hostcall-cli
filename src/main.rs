mod cli;
use cli::{Action::*, CommandLineArgs};
use std::io;
use structopt::StructOpt;
mod load_module;
use load_module::*;

fn main() {
    load_module::initialize().unwrap();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut args: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        args.insert(0, String::new());
        let args = CommandLineArgs::from_iter_safe(&args);
        match args {
            Ok(args) => match args.action {
                Load { module_name } => {
                    println!("Loading module {}", module_name);
                    load_module_by_name(module_name).unwrap();
                }
                Unload { module_name } => {
                    println!("Unloading module {}", module_name);
                    unload_module_by_name(module_name).unwrap();
                }
                List => {
                    println!("Loaded modules list:");
                    show_module_list().unwrap();
                }
                Listfn { module_name } => {
                    println!("Listing functions in module {}", module_name);
                }
                Call {
                    module_name,
                    function_name,
                    args,
                } => {
                    println!(
                        "Calling function {} in module {} with args {:?}",
                        function_name, module_name, args
                    );
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
