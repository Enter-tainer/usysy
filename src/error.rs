use miette::{Diagnostic, NamedSource, SourceSpan};
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
  #[error("unknown type")]
  #[diagnostic()]
  UnknownType(),
  #[error("duplicate global symbol")]
  #[diagnostic()]
  DuplicateSymbol {
    #[source_code]
    src: NamedSource,
    #[label("duplicate symbol here")]
    range: SourceSpan,
  },
  #[error("missing variable")]
  #[diagnostic()]
  VariableNotFound {
    #[source_code]
    src: NamedSource,
    #[label("this variable cannot be found")]
    range: SourceSpan,
  },
  #[error("keyword not in a loop")]
  #[diagnostic()]
  KeywordNotInLoop {
    #[source_code]
    src: NamedSource,
    #[label("this keyword can only use in a loop")]
    range: SourceSpan,
  },
}
pub type Result<T> = std::result::Result<T, Error>;
