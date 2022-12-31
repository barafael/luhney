use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    #[arg(short, long, value_parser)]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    dbg!(args);
}
