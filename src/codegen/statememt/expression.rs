use crate::error::Result;
use tree_sitter::Node;

use super::Generator;
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_expression_statement(&mut self, _root: Node) -> Result<()> {
    // do nothing, because single expr has no effect.
    Ok(())
  }
}
