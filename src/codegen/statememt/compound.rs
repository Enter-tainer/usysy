use crate::{error::Result, parser::useful_children};
use tree_sitter::Node;

use super::Generator;
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_compound_statement(&mut self, root: Node) -> Result<()> {
    let mut cursor = root.walk();
    for i in useful_children(&root, &mut cursor) {
      self.generate_statement(i)?;
    }
    Ok(())
  }
}
