use clap::Parser as ClapParser;
use std::path::PathBuf;

#[derive(ClapParser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Glob pattern(s) for input files. Repeat to pass multiple patterns.
    /// Omit to read from stdin.
    /// Examples: --input "src/**/*.html"  --input "templates/*.html"
    #[arg(short, long)]
    pub(crate) input: Vec<String>,

    /// Output file path (only used when exactly one input file is matched).
    /// When multiple files match the glob, each file is formatted in-place.
    /// Omit to write to stdout (single-file / stdin mode).
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
