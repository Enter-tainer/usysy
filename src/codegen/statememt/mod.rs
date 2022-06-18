mod assignment;
mod compound;
mod expression;
mod if_statement;
mod while_statement;

use crate::error::Result;
use tree_sitter::Node;

use super::{BaseType, Generator};
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_statement(&mut self, root: Node) -> Result<()> {
    let stat_type = root.kind();
    match stat_type {
      "compound_statement" => self.generate_compound_statement(root)?,
      "expression_statement" => self.generate_expression_statement(root)?,
      "if_statement" => self.generate_if_statement(root)?,
      "while_statement" => self.generate_while_statement(root)?,
      "break_statement" => {}
      "continue_statement" => {}
      "return_statement" => {}
      "assignment" => self.generate_assignment_statement(root)?,
      "declaration" => self.generate_local_var(root)?,
      _ => unreachable!("unknown statement type {stat_type}"),
    }
    Ok(())
  }
}
