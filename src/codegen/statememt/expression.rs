use crate::error::Result;
use tree_sitter::Node;

use super::Generator;
impl<'ctx> Generator<'ctx> {
  pub(super) fn generate_expression_statement(&mut self, root: Node) -> Result<()> {
    self.generate_expression(root.child(0).unwrap())?;
    Ok(())
  }
}
