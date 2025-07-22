use std::io::stdin;
use std::path::PathBuf;

use clap::Parser;

use fellow::interpret;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser, value_name = "SCRIPT")]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.path);

    match args.path {
        Some(path) => run_script(&path),
        None => run_interactive(),
    }
}

fn run_script(path: &PathBuf) {
    match std::fs::read_to_string(path) {
        Ok(contents) => match interpret(&contents) {
            Ok(value) => println!("{}", value),
            Err(err) => eprintln!("Error {:?}", err),
        },
        Err(err) => eprintln!("Failed to read source code from {:?}", err),
    }
}

fn run_interactive() {
    loop {
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_size) => match interpret(&buffer) {
                Ok(value) => println!("{}", value),
                Err(err) => eprintln!("Error {:?}", err),
            },
            Err(err) => eprintln!("Failed to read line {:?}", err),
        }
    }
}
