use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum};
use itertools::Itertools;
use miette::NamedSource;
use tree_sitter::Node;

use super::{BaseType, Generator};
use crate::{
  error::{Error, Result},
  parser::{get_text, to_source_span, useful_children},
};
impl<'ctx> Generator<'ctx> {
  pub(super) fn generate_call_expression(
    &self,
    root: Node,
  ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    let fn_node = root.child_by_field_name("function").unwrap();
    let fn_name = get_text(fn_node, self.file.content);
    let params = root.child_by_field_name("arguments").unwrap();
    let params_expr: Vec<(BaseType, BasicValueEnum)> = {
      let mut cursor = params.walk();
      useful_children(&params, &mut cursor)
        .map(|expr| self.generate_expression(expr))
        .try_collect()?
    };
    let (fn_ret_ty, _params, _is_va_arg) =
      self
        .function_map
        .get(fn_name)
        .ok_or(Error::FunctionNotFound {
          src: NamedSource::new(self.file.name, self.file.content.to_string()),
          range: to_source_span(fn_node.range()),
        })?;
    let fn_val = self.module.get_function(fn_name).unwrap();
    let ret_v = self
      .builder
      .build_call(
        fn_val,
        &params_expr
          .iter()
          .map(|(_, val)| BasicMetadataValueEnum::from(*val))
          .collect_vec(),
        "fn_call",
      )
      .try_as_basic_value()
      .left();
    if fn_ret_ty.base_type == BaseType::Void
      || fn_ret_ty.base_type != BaseType::Void && ret_v.is_some()
    {
      Ok((
        fn_ret_ty.base_type.clone(),
        ret_v.unwrap_or_else(|| self.context.i32_type().const_zero().as_basic_value_enum()),
      ))
    } else {
      unreachable!()
    }
  }
}
