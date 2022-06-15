use miette::Diagnostic;
use thiserror::Error;
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
  #[error("transparent")]
  #[diagnostic(help("error with tree_sitter parsing library"))]
  TreeSitter(#[from] tree_sitter::LanguageError),
  #[error("treesitter parse failed")]
  #[diagnostic()]
  TreesitterParseFailed,
  #[error("io error")]
  #[diagnostic()]
  IO(#[from] std::io::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
