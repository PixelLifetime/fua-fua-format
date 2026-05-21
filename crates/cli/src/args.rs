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

    /// Format only git-staged files that match config include/exclude rules.
    /// Intended for pre-commit hooks (default Husky behavior).
    #[arg(long, conflicts_with_all = ["all", "changed"])]
    pub(crate) only_staged: bool,

    /// Format all project files that match config include/exclude rules.
    #[arg(long, conflicts_with_all = ["only_staged", "changed"])]
    pub(crate) all: bool,

    /// Format files changed on the current branch (vs merge base). For CI on pull requests.
    /// Base ref: `GITHUB_BASE_REF` / `FUA_BASE_REF`, else `origin/master`.
    #[arg(long, conflicts_with_all = ["only_staged", "all"])]
    pub(crate) changed: bool,

    /// Verify files are formatted; do not write. Exits with an error when changes are needed.
    #[arg(long)]
    pub(crate) check: bool,
}
