use crate::{
  error::Result,
  parser::{get_text, useful_children},
};
use inkwell::values::BasicValueEnum;
use tree_sitter::Node;

use super::{BaseType, Generator};
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_assignment_statement(&self, root: Node) -> Result<()> {
    let lhs = root.child_by_field_name("left").unwrap();
    let rhs = root.child_by_field_name("right").unwrap();
    if lhs.kind() == "subscript_expression" {
      todo!("array element assignment not supported!");
    }
    let lhs_str = get_text(lhs, self.file.content);
    let (lhs_ty, lhs_var) = self.get_in_value_map(lhs_str, lhs.range())?;
    let (rhs_ty, rhs_var) = self.generate_expression(rhs)?;
    self.builder.build_store(lhs_var, rhs_var);
    Ok(())
  }
}
