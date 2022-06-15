mod cli;
mod error;
mod parser;
use clap::Parser;
use cli::Args;
use error::Result;
use parser::{dump_node, parse};
fn main() -> Result<()> {
    let Args { input, ast } = cli::Args::parse();
    let file = std::fs::read_to_string(input)?;
    let tree = parse(&file)?;
    if ast {
        dump_node(&tree.root_node(), &file);
    }
    Ok(())
}
