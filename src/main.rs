use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser, value_name = "SCRIPT")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.path);

    if let source_code = read_source_code(&args.path) {
    } else {
        eprintln!("Failed to read source code from {:?}", args.path)
    }
}

enum FellowError {
    CannotReadFile(String),
}

fn read_source_code(path: &PathBuf) -> Result<String, FellowError> {}
