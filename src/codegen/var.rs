use crate::{
  error::{Error, Result},
  parser::{get_text, to_source_span, useful_children},
};
use inkwell::values::BasicValueEnum;
use itertools::Itertools;
use miette::NamedSource;
use tree_sitter::Node;

use super::{BaseType, Generator, MBasicType};

impl<'ctx> Generator<'ctx> {
  fn generate_var_impl(
    &mut self,
    is_const: bool,
    mut ty: BaseType,
    declarator: Node,
    is_global: bool,
  ) -> Result<()> {
    let name = declarator.child_by_field_name("name").unwrap();

    let name_str = get_text(
      declarator.child_by_field_name("name").unwrap(),
      self.file.content,
    );
    if is_global {
      if self.function_map.contains_key(name_str)
        || self.val_map_block_stack[0].contains_key(name_str)
      {
        return Err(Error::DuplicateSymbol {
          src: NamedSource::new(self.file.name, self.file.content.to_string()),
          range: to_source_span(name.range()),
        });
      }
    } else if self
      .val_map_block_stack
      .last()
      .unwrap()
      .contains_key(name_str)
    {
      return Err(Error::DuplicateSymbol {
        src: NamedSource::new(self.file.name, self.file.content.to_string()),
        range: to_source_span(name.range()),
      });
    }
    if let Some(dimension) = declarator.child_by_field_name("dimension") {
      let dimensions = {
        let mut cursor = dimension.walk();
        useful_children(&dimension, &mut cursor)
          .map(|i| {
            assert_eq!(i.kind(), "int_literal");
            let text = get_text(i, self.file.content);
            parse_int::parse::<i32>(text).unwrap()
          })
          .collect_vec()
      };
      ty = BaseType::Array(Box::new(ty.clone()), dimensions);
    }
    let llvm_type = ty.to_llvm_type(self.context);

    let init = declarator.child_by_field_name("init");
    let initializer = if let Some(init) = init {
      let mut cursor = init.walk();
      let init = init.children(&mut cursor).find(|c| c.kind() != "comment").unwrap();
      match init.kind() {
        "init_list" => self.generate_array_init_list(init, &ty)?,
        "empty_init_list" => {
          todo!()
        }
        _ => {
          let (expr_ty, val) = self.generate_expression(init)?;
          let casted = self.cast_value(&expr_ty, &val, &ty, declarator.range())?;
          casted
        }
      }
    } else {
      llvm_type.const_zero()
    };
    if is_global {
      let global_value = self.module.add_global(llvm_type, None, name_str);
      // global_value.set_linkage(Linkage::Common);
      // if is_const {
      //   global_value.set_constant(true);
      // }
      global_value.set_initializer(&initializer);
      self.val_map_block_stack[0].insert(
        name_str.to_string(),
        (
          MBasicType {
            is_const,
            base_type: ty,
          },
          global_value.as_pointer_value(),
        ),
      );
    } else {
      let local_value = self.builder.build_alloca(llvm_type, name_str);
      self.builder.build_store(local_value, initializer);
      self.insert_to_val_map(
        &MBasicType {
          is_const,
          base_type: ty,
        },
        name_str,
        local_value,
        declarator.range(),
      )?;
    }
    Ok(())
  }

  fn generate_array_init_list(
    &self,
    init_list: Node,
    array_ty: &BaseType,
  ) -> Result<BasicValueEnum> {
    todo!()
  }

  pub(super) fn generate_global_var(&mut self, root: Node) -> Result<()> {
    let ty: BaseType =
      get_text(root.child_by_field_name("type").unwrap(), self.file.content).try_into()?;
    let is_const = root.child_by_field_name("const").is_some();
    let declarators = {
      let mut cursor = root.walk();
      useful_children(&root, &mut cursor)
        .filter(|node| node.kind() == "declarator")
        .collect_vec()
    };
    for declarator in declarators {
      self.generate_var_impl(is_const, ty.clone(), declarator, true)?;
    }
    Ok(())
  }
  pub(super) fn generate_local_var(&mut self, root: Node) -> Result<()> {
    let ty: BaseType =
      get_text(root.child_by_field_name("type").unwrap(), self.file.content).try_into()?;
    let is_const = root.child_by_field_name("const").is_some();
    let declarators = {
      let mut cursor = root.walk();
      useful_children(&root, &mut cursor)
        .filter(|node| node.kind() == "declarator")
        .collect_vec()
    };
    for declarator in declarators {
      self.generate_var_impl(is_const, ty.clone(), declarator, false)?;
    }
    Ok(())
  }
}
