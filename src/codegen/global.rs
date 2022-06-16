use crate::{
  error::{Error, Result},
  parser::useful_children,
};
use tree_sitter::Node;

use super::Generator;

impl<'ctx> Generator<'ctx> {
  pub(super) fn generate_global_proto(&mut self, root: Node) -> Result<()> {
    let mut cursor = root.walk();
    for node in useful_children(&root, &mut cursor) {
      if node.kind() == "function_definition" {
        self.generate_function_proto(node)?;
      }
    }
    Ok(())
  }

  pub(super) fn generate_global_definition(&mut self, root: Node) -> Result<()> {
    let mut cursor = root.walk();
    for node in useful_children(&root, &mut cursor) {
      if node.kind() == "function_definition" {
        self.generate_function_definition(node)?;
      }
    }
    Ok(())
  }
}
