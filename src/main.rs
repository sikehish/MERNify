use clap::{App, Arg};
use std::fs;
use std::path::Path;
use std::process::Command;
use clap::{App, Arg};
use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("MERN Stack App Generator")
        .version("1.0")
        .author("Hisham Akmal")
        .about("Generates a MERN stack app")
        .arg(
            Arg::with_name("project_name")
                .short("p")
                .long("project")
                .value_name("PROJECT_NAME")
                .required(true)
                .help("Sets the project name")
                .takes_value(true),
        )
        .get_matches();

}


