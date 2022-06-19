mod cli;
mod codegen;
mod error;
mod parser;
use std::{path::Path, process::Command};

use clap::Parser;
use cli::Args;
use codegen::Generator;
use inkwell::context::Context;
use miette::{IntoDiagnostic, Result};
use parser::{dump_node, parse};
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
  let bc_path = format!("{}.bc", base.file_stem().unwrap().to_str().unwrap());
  let exe_path = format!("{}.exe", base.file_stem().unwrap().to_str().unwrap());
  if ir_enable || exe_enable {
    gen.write(&bc_path);
  }
  if ir_enable {
    Command::new("llvm-dis").arg(&bc_path).output().unwrap();
  }
  if exe_enable {
    Command::new("clang")
      .args([
        &bc_path,
        "./compiler2022/runtime/sylib.c",
        &format!("-o{}", exe_path),
      ])
      .spawn()
      .unwrap();
  }

  Ok(())
}
