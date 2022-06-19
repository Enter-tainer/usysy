use inkwell::values::{BasicValue, BasicValueEnum};
use lexical::{format::STANDARD, parse_with_options};
use miette::NamedSource;
use tree_sitter::Node;

use super::{BaseType, Generator};
use crate::{
  error::{Error, Result},
  parser::{get_text, to_source_span},
};
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_float_literal_expression(
    &self,
    root: Node,
  ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    let lit = get_text(root, self.file.content);
    let lit = parse_with_options::<f32, _, STANDARD>(
      lit.as_bytes(),
      &lexical::parse_float_options::C_LITERAL,
    )
    .map_err(|_| Error::ParseLiteralFailed {
      src: NamedSource::new(self.file.name, self.file.content.to_string()),
      range: to_source_span(root.range()),
    })?;
    Ok((
      BaseType::Int,
      self
        .context
        .f32_type()
        .const_float(lit as f64)
        .as_basic_value_enum(),
    ))
  }
  pub(super) fn generate_int_literal_expression(
    &self,
    root: Node,
  ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    let lit = get_text(root, self.file.content);
    let lit: i32 = parse_int::parse(lit).map_err(|_| Error::ParseLiteralFailed {
      src: NamedSource::new(self.file.name, self.file.content.to_string()),
      range: to_source_span(root.range()),
    })?;
    Ok((
      BaseType::Int,
      self
        .context
        .i32_type()
        .const_int(lit as u64, true)
        .as_basic_value_enum(),
    ))
  }
}
