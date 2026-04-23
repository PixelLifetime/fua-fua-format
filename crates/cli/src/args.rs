use clap::Parser as ClapParser;
use std::path::PathBuf;

#[derive(ClapParser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Input file path (or stdin if not provided)
    #[arg(short, long)]
    pub(crate) input: Option<PathBuf>,

    /// Output file path (or stdout if not provided)
    #[arg(short, long)]
    pub(crate) output: Option<PathBuf>,

    /// Path to a JSON configuration file
    #[arg(short, long)]
    pub(crate) config: Option<PathBuf>,

    /// Override indent size
    #[arg(long)]
    pub(crate) indent_size: Option<usize>,

    /// Use tabs instead of spaces
    #[arg(long)]
    pub(crate) use_tabs: Option<bool>,

    /// Path to a compiled .wasm formatter plugin. Repeat to load multiple plugins.
    #[arg(long)]
    pub(crate) plugin: Vec<PathBuf>,
}
