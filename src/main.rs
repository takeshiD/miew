use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Destination {
    Terminal,
    Browser,
    File,
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    src: PathBuf,
    #[arg(value_enum, default_value_t = Destination::Terminal)]
    dst: Destination
}
mod render;
fn main() {
    let args = Args::parse();
    use markdown::{to_mdast, ParseOptions};
    use std::fs::File;
    use std::io::prelude::Read;
    let mut src = File::open(args.src).expect("file not found");
    let mut content = String::new();
    src.read_to_string(&mut content).unwrap();
    println!("{:#?}", to_mdast(content.as_str(), &ParseOptions::default()).unwrap());
}
