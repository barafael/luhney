use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    #[arg(short, long, value_parser)]
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut lines = read_lines(args.input)?;
    while let Some(Ok(line)) = lines.next() {
        if luhney::luhn(&line) {
            println!("\"{line}\": valid");
        } else {
            println!("\"{line}\": invalid");
        }
    }
    Ok(())
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
