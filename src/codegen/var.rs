use crate::error::{Error, Result};
use tree_sitter::Node;

use super::Generator;

impl<'ctx> Generator<'ctx> {
  pub(super) fn generate_global_var(&mut self, root: Node) -> Result<()> {
    todo!()
  }
}
