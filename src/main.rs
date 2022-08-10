mod cli;
use cli::{Action::*, CommandLineArgs};
use std::io;
use structopt::StructOpt;
mod load_module;
use load_module::*;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut args: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        args.insert(0, String::new());
        let args = CommandLineArgs::from_iter_safe(&args);
        match args {
            Ok(args) => match args.action {
                Load { path } => {
                    println!("Loading module {}", path);
                    load_module_by_path(path);
                }
                Unload { module_name } => {
                    println!("Unloading module {}", module_name);
                }
                List => {
                    println!("Listing modules");
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
