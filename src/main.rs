mod cli;
use std::{path::Path, process::Command};

use clap::Parser;
use cli::Args;
use inkwell::context::Context;
use miette::{IntoDiagnostic, Result};
use sysy::parser::{dump_node, parse};
use sysy::{
  codegen::Generator,
  util::{compile_with_clang, get_bc_exe_path},
};
fn main() -> Result<()> {
  let Args {
    input,
    ast,
    prototype,
    global,
    ir_enable,
    exe_enable,
  } = cli::Args::parse();
  let file = std::fs::read_to_string(&input).into_diagnostic()?;
  let tree = parse(&file)?;
  if ast {
    dump_node(&tree.root_node(), &file);
  }
  let ctx = Context::create();
  let mut gen = Generator::new(&ctx, &input, &file);
  gen.gen(&tree)?;
  if prototype {
    gen.print_function_proto();
  }
  if global {
    gen.print_global_var();
  }
  let base = Path::new(&input);
  let (bc_path, exe_path) = get_bc_exe_path(base);
  if ir_enable || exe_enable {
    gen.write(&bc_path);
  }
  if ir_enable {
    Command::new("llvm-dis").arg(&bc_path).output().unwrap();
  }
  if exe_enable {
    compile_with_clang(&bc_path,&exe_path);
  }

  Ok(())
}
