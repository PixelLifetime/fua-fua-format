use clap::Parser as ClapParser;
use fua_core::config::FormatterConfig;
use fua_core::formatter::Formatter;
use fua_core::parser::Parser;
use fua_core::syntax::SyntaxNode;
use std::fs;
use std::io::{self, Read};

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path (or stdin if not provided)
    #[arg(short, long)]
    input: Option<String>,

    /// Output file path (or stdout if not provided)
    #[arg(short, long)]
    output: Option<String>,

    /// Path to a JSON configuration file
    #[arg(short, long)]
    config: Option<String>,

    /// Override indent size
    #[arg(long)]
    indent_size: Option<usize>,

    /// Use tabs instead of spaces
    #[arg(long)]
    use_tabs: Option<bool>,
}

fn main() {
    let args = Args::parse();
    
    // Read input
    let mut input_text = String::new();
    if let Some(path) = &args.input {
        input_text = fs::read_to_string(path).expect("Failed to read input file");
    } else {
        io::stdin().read_to_string(&mut input_text).expect("Failed to read stdin");
    }

    if input_text.trim().is_empty() {
        eprintln!("Error: No HTML provided.");
        std::process::exit(1);
    }

    // Load configuration
    let mut config = FormatterConfig::default();
    if let Some(config_path) = &args.config {
        let config_str = fs::read_to_string(config_path).expect("Failed to read config file");
        config = serde_json::from_str(&config_str).expect("Failed to parse config file");
    }

    // Apply CLI overrides over JSON config
    if let Some(size) = args.indent_size {
        config.indent_size = size;
    }
    if let Some(tabs) = args.use_tabs {
        config.use_tabs = tabs;
    }

    // Parse constraints securely via Lossless parser maps
    let parser = Parser::new(&input_text);
    let green_node = parser.parse();
    let syntax_node = SyntaxNode::new_root(green_node);

    // Apply formatting walker 
    let formatter = Formatter::new(config);
    let output_text = formatter.format(&syntax_node);

    // Write output dynamically to File natively or STDOUT bounds
    if let Some(path) = &args.output {
        fs::write(path, output_text).expect("Failed to write output file");
        println!("Successfully formatted into: {}", path);
    } else {
        print!("{}", output_text);
    }
}
