use crate::{error::Result, parser::useful_children};
use inkwell::values::BasicValueEnum;
use tree_sitter::Node;

use super::{BaseType, Generator};
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_if_statement(&mut self, root: Node) -> Result<()> {
    Ok(())
  }
}
