use inkwell::values::{BasicValue, BasicValueEnum};
use tree_sitter::Node;

use super::{BaseType, Generator};
use crate::{error::Result, parser::get_text};
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_identifier_expression(
    &self,
    root: Node,
  ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    let var_name = get_text(root, self.file.content);
    let var = self.get_in_value_map(var_name, root.range())?;
    Ok((var.0.base_type, var.1.as_basic_value_enum()))
  }
}
