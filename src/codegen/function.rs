use crate::error::{Error, Result};
use tree_sitter::Node;

use super::Generator;

impl<'ctx> Generator<'ctx> {
  pub(super) fn generate_function_proto(&mut self, function: Node) -> Result<()> {
    todo!()
  }

  pub(super) fn generate_function_definition(&mut self, function: Node) -> Result<()> {
    todo!()
  }
}
