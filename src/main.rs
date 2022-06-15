mod cli;
mod error;
mod parser;
use clap::Parser;
use cli::Args;
use miette::{IntoDiagnostic, Result};
use parser::{dump_node, parse};
fn main() -> Result<()> {
    let Args { input, ast } = cli::Args::parse();
    let file = std::fs::read_to_string(input).into_diagnostic()?;
    let tree = parse(&file).into_diagnostic()?;
    if ast {
        dump_node(&tree.root_node(), &file);
    }
    Ok(())
}
