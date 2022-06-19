use inkwell::{
  values::{BasicValue, BasicValueEnum},
  IntPredicate,
};
use tree_sitter::Node;

use super::{BaseType, Generator};
use crate::{error::Result, parser::get_text};
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_unary_expression(
    &self,
    root: Node,
  ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    let op = root.child_by_field_name("operator").unwrap();
    let argument = root.child_by_field_name("argument").unwrap();
    let (ty, val) = self.generate_expression(argument)?;
    Ok(match get_text(op, self.file.content) {
      "+" => (ty, val),
      "-" => (
        ty.clone(),
        match ty {
          BaseType::Int => self
            .builder
            .build_int_neg(val.into_int_value(), "int_neg")
            .as_basic_value_enum(),
          BaseType::Float => self
            .builder
            .build_float_neg(val.into_float_value(), "float_neg")
            .as_basic_value_enum(),
          _ => todo!(),
        },
      ),
      "!" => (
        BaseType::Int,
        match ty {
          BaseType::Int => {
            let result_int = self.builder.build_int_compare(
              IntPredicate::EQ,
              self.context.i32_type().const_int(0_u64, true),
              val.into_int_value(),
              "logical_not_result_i1",
            );
            let result_int_i32 = self.builder.build_int_cast(result_int, self.context.i32_type(), "logical_not_result_i32");
            result_int_i32.as_basic_value_enum()
          }
          _ => todo!(),
        },
      ),
      op => unreachable!("unknown unary expr {op}"),
    })
  }
}
