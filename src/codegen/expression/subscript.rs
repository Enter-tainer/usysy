use inkwell::values::{BasicValue, BasicValueEnum, PointerValue};
use itertools::Itertools;
use tree_sitter::Node;

use super::{BaseType, Generator};
use crate::{
  codegen::MBasicType,
  error::Result,
  parser::{get_text, useful_children},
};
impl<'a, 'ctx> Generator<'ctx> {
  fn generate_subscript_indices(&self, root: Node) -> Result<Vec<BasicValueEnum<'ctx>>> {
    // let subscripts = self.collect_continuous_subscript(root)?;
    let mut cursor = root.walk();
    let res = useful_children(&root, &mut cursor)
      .map(|child| self.generate_expression(child).map(|(_ty, val)| val))
      .try_collect()?;
    Ok(res)
  }

  fn generate_subscript_expression_inner(
    &self,
    root: Node,
  ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    let array_name = root.child_by_field_name("argument").unwrap();
    let index = root.child_by_field_name("indices").unwrap();
    let indices = std::iter::once(self.context.i32_type().const_zero())
      .chain(
        self
          .generate_subscript_indices(index)?
          .into_iter()
          .map(|x| x.into_int_value()),
      )
      .collect_vec();
    let array_name_str = get_text(array_name, self.file.content);
    let (arr_ty, arr_val) = self.get_in_value_map(array_name_str, array_name.range())?;
    let res_ptr = unsafe {
      self
        .builder
        .build_gep(arr_val, &indices, "get array address")
    };
    Ok((
      arr_ty.base_type.get_elem_type(),
      res_ptr.as_basic_value_enum(),
    ))
  }
  pub(super) fn generate_subscript_expression(
    &self,
    root: Node,
  ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    let (ty, res_ptr) = self.generate_subscript_expression_inner(root)?;
    let res = self
      .builder
      .build_load(res_ptr.into_pointer_value(), "deref to load array var");
    Ok((ty, res))
  }
  pub(crate) fn generate_subscript_expression_lv(
    &self,
    root: Node,
  ) -> Result<(MBasicType, PointerValue<'ctx>)> {
    let (ty, res_ptr) = self.generate_subscript_expression_inner(root)?;
    Ok((
      MBasicType::new_with_base_mut(ty),
      res_ptr.into_pointer_value(),
    ))
  }
}
