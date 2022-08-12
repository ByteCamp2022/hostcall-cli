use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Action {
    /// Load a module
    Load {
        #[structopt()]
        path: String,
        name: String,
    },
    /// Unload a module
    Unload {
        #[structopt()]
        module_name: String,
    },
    /// List loaded modules
    List,
    /// Call a function in a module
    Call {
        #[structopt()]
        module_name: String,
        #[structopt()]
        function_name: String,
        #[structopt()]
        param: String,
    },
    /// Send a request to the server
    Request {
        #[structopt()]
        url: String,
        #[structopt()]
        port: String,
        #[structopt()]
        path: String,
    },
    /// Exits the program
    Exit,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "terminal-hostcall",
    about = "A simple CLI for interacting with the hostcall module"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}