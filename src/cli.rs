use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
  #[clap(help("input file path"))]
  pub input: String,
  #[clap(short, long, help("print ast"))]
  pub ast: bool,
  #[clap(short, long, help("print function prototypes"))]
  pub prototype: bool,
  #[clap(short, long, help("print global vars"))]
  pub global: bool,
  #[clap(short, long, help("enable ir output"))]
  pub ir_enable: bool,
  #[clap(short, long, help("enable exe output"))]
  pub exe_enable: bool,
}
