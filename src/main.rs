use std::fs;
// use structopt::StructOpt; //StructOpt trait->structopt crate, used parsing command-line arguments.
use clap::Parser; //using Clap as an alternative for structOpt
use std::path::PathBuf; //PathBuf type -> std::path module, which is used to manipulate paths.

#[derive(Parser,Default,Debug)]
// #[derive(StructOpt)] // deriving StructOpt trait(implements the StructOpt trait for the Cli struct) - https://doc.rust-lang.org/rust-by-example/trait/derive.html
struct Cli {
    // #[structopt(parse(from_os_str))]
    path: PathBuf, //path field is  parsed from a command-line argument and converted into a PathBuf
}

fn main() {
    let args = Cli::parse();
    println!("Path: {:?}", args.path);
}

//Run command: cargo run -- pathname

