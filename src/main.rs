use std::fs;
use structopt::StructOpt; //StructOpt trait->structopt crate, used parsing command-line arguments.
use std::path::PathBuf; //PathBuf type -> std::path module, which is used to manipulate paths.

#[derive(StructOpt)] // deriving StructOpt trait(implements the StructOpt trait for the Cli struct) - https://doc.rust-lang.org/rust-by-example/trait/derive.html
struct Cli {
    #[structopt(parse(from_os_str))]
    path: PathBuf, //path field is  parsed from a command-line argument and converted into a PathBuf
}

fn main() {
    let args = Cli::from_args();
    println!("Path: {:?}", args.path);
}
