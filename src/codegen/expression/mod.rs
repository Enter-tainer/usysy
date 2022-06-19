mod binary;
mod call;
mod identifier;
mod literal;
mod subscript;
mod unary;
use inkwell::values::{BasicValue, BasicValueEnum};
use tree_sitter::Node;

use super::{BaseType, Generator};
use crate::error::Result;
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub fn generate_expression(&self, root: Node) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    match root.kind() {
      "binary_expression" => self.generate_binary_expression(root),
      "unary_expression" => self.generate_unary_expression(root),
      "subscript_expression" => self.generate_subscript_expression(root),
      "call_expression" => self.generate_call_expression(root),
      "identifier" => self.generate_identifier_expression(root),
      "float_literal" => self.generate_float_literal_expression(root),
      "int_literal" => self.generate_int_literal_expression(root),
      "parenthesized_expression" => {
        let child = root.named_child(0).unwrap();
        self.generate_expression(child)
      }
      ";" => Ok((
        BaseType::Int,
        self.context.i32_type().const_zero().as_basic_value_enum(),
      )),
      _ => unreachable!("unknown expression kind {}", root.kind()),
    }
  }
}
