mod app;
mod args;
mod file_selection;

use args::Args;
use clap::Parser as _;

fn main() {
    let args = Args::parse();

    if let Err(error) = app::run(args) {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
