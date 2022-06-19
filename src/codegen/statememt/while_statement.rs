use crate::error::Result;
use tree_sitter::Node;

use super::Generator;
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_while_statement(&mut self, root: Node) -> Result<()> {
    let current_fn = self.current_function.as_ref().unwrap().0;

    let cond = root.child_by_field_name("condition").unwrap();
    let body = root.child_by_field_name("body").unwrap();
    let before_loop_lock = self.context.append_basic_block(current_fn, "before_loop");
    let loop_body_block = self.context.append_basic_block(current_fn, "loop_body");
    let after_loop_block = self.context.append_basic_block(current_fn, "after_loop");
    self.break_labels.push_back(after_loop_block);
    self.continue_labels.push_back(before_loop_lock);
    self.builder.position_at_end(before_loop_lock);
    let cond_expr = self.generate_expression(cond)?.1.into_int_value();
    self
      .builder
      .build_conditional_branch(cond_expr, loop_body_block, after_loop_block);
    self.builder.position_at_end(loop_body_block);
    self.generate_statement(body)?;
    self.builder.build_unconditional_branch(after_loop_block);
    self.builder.position_at_end(after_loop_block);
    self.break_labels.pop_back();
    self.continue_labels.pop_back();
    Ok(())
  }
}
