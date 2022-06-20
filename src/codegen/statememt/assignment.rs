use crate::{error::Result, parser::get_text};

use tree_sitter::Node;

use super::Generator;
impl<'ctx> Generator<'ctx> {
  pub(super) fn generate_assignment_statement(&self, root: Node) -> Result<()> {
    let lhs = root.child_by_field_name("left").unwrap();
    let rhs = root.child_by_field_name("right").unwrap();
    let lhs_str = get_text(lhs, self.file.content);
    let (_lhs_ty, lhs_var) = if lhs.kind() == "subscript_expression" {
      self.generate_subscript_expression_lv(lhs)?
    } else {
      self.get_in_value_map(lhs_str, lhs.range())?
    };
    let (_rhs_ty, rhs_var) = self.generate_expression(rhs)?;
    self.builder.build_store(lhs_var, rhs_var);
    Ok(())
  }
}
