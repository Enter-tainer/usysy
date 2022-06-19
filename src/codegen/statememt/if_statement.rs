use crate::error::Result;
use tree_sitter::Node;

use super::Generator;
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_if_statement(&mut self, root: Node) -> Result<()> {
    let current_fn = self.current_function.as_ref().unwrap().0;

    let consequence_block = self.context.append_basic_block(current_fn, "if_block");
    let alternative_block = self.context.append_basic_block(current_fn, "else_block");
    let after_block = self.context.append_basic_block(current_fn, "after_block");
    let cond = root.child_by_field_name("condition").unwrap();
    let consequence = root.child_by_field_name("consequence").unwrap();
    let alternative = root.child_by_field_name("alternative");
    let cond_expr = self.generate_expression(cond)?.1.into_int_value();
    self
      .builder
      .build_conditional_branch(cond_expr, consequence_block, alternative_block);
    self.builder.position_at_end(consequence_block);
    self.generate_statement(consequence)?;
    self.builder.build_unconditional_branch(after_block);
    if let Some(alternative) = alternative {
      self.builder.position_at_end(alternative_block);
      self.generate_statement(alternative)?;
      self.builder.build_unconditional_branch(after_block);
    }
    self.builder.position_at_end(after_block);
    Ok(())
  }
}
