use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(help("input file path"))]
    pub input: String,
    #[clap(short, long, help("print ast"))]
    pub ast: bool,
}
