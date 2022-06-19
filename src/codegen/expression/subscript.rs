use inkwell::values::BasicValueEnum;
use tree_sitter::Node;

use super::{BaseType, Generator};
use crate::error::Result;
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_subscript_expression(
    &self,
    root: Node,
  ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    Ok((
      BaseType::Int,
      BaseType::Int.to_llvm_type(self.context).const_zero(),
    ))
  }
}
