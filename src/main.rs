mod cli;
mod codegen;
mod error;
mod parser;
use clap::Parser;
use cli::Args;
use codegen::Generator;
use inkwell::context::Context;
use miette::{IntoDiagnostic, Result};
use parser::{dump_node, parse};
fn main() -> Result<()> {
  let Args { input, ast } = cli::Args::parse();
  let file = std::fs::read_to_string(&input).into_diagnostic()?;
  let tree = parse(&file)?;
  if ast {
    dump_node(&tree.root_node(), &file);
  }
  let ctx = Context::create();
  let mut gen = Generator::new(&ctx, &input, &file);
  gen.gen(&tree)?;
  gen.write("res.bc");
  Ok(())
}
