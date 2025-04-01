use std::path::PathBuf;

use clap::Parser;

use fellow::interpret;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser, value_name = "SCRIPT")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.path);

    match std::fs::read_to_string(&args.path) {
        Ok(contents) => match interpret(&contents) {
            Ok(value) => println!("{}", value),
            Err(err) => eprintln!("Error {:?}", err),
        },
        Err(err) => eprintln!("Failed to read source code from {:?}", err),
    }
}
