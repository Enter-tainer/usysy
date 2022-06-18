use crate::{
  error::{Error, Result},
  parser::{get_text, to_source_span, useful_children},
};
use inkwell::{module::Linkage, values::PointerValue};
use itertools::Itertools;
use miette::NamedSource;
use tree_sitter::{Node, Range};

use super::{BaseType, Generator, MBasicType};

impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn insert_to_val_map(
    &mut self,
    var_type: &MBasicType<'node>,
    identifier: &str,
    ptr: PointerValue<'ctx>,
    range: Range
  ) -> Result<()> {
    let local_map = self.val_map_block_stack.last_mut().unwrap();

    if local_map.contains_key(identifier) {
      return Err(Error::DuplicateSymbol {
        src: NamedSource::new(self.file.name, self.file.content.to_string()),
        range: to_source_span(range),
      });
    }

    local_map.insert(identifier.to_string(), (var_type.clone(), ptr));
    Ok(())
  }
}
