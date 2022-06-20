use std::collections::HashMap;

use crate::{error::Result, parser::useful_children};
use tree_sitter::Node;

use super::Generator;
impl<'ctx> Generator<'ctx> {
  pub(super) fn generate_compound_statement(&mut self, root: Node) -> Result<()> {
    let mut cursor = root.walk();
    self.val_map_block_stack.push(HashMap::new());
    for i in useful_children(&root, &mut cursor) {
      self.generate_statement(i)?;
    }
    self.val_map_block_stack.pop();
    Ok(())
  }
}
