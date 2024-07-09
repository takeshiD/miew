mod parser;
mod renderer;
mod drawer;
mod config;

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
    dst: Destination,
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let src = args.src;
    let dst = args.dst;
    let mut config = Config::default();
    if let Some(config_path) = args.config {
        match config.load(config_path) {
            Ok(()) => (),
            Err(e) => panic!("Failed open '{config_path}'. speacified path is not exisited or allow permission."),
        }
    }
    let mdast = parser::Parser(config);
    let rendertree = renderer::Renderer(config);

}
