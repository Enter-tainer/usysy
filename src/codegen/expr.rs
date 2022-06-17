use crate::{error::Result, parser::useful_children};
use inkwell::values::BasicValueEnum;
use tree_sitter::Node;

use super::{BaseType, Generator};
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub fn generate_expression(&mut self, root: Node) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    Ok((
      BaseType::Int,
      BaseType::Int.to_llvm_type(self.context).const_zero(),
    ))
  }
}
