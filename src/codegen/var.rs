use crate::error::{Error, Result};
use tree_sitter::Node;

use super::Generator;

impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_global_var(&mut self, root: Node) -> Result<()> {
    
    Ok(())
  }
}
