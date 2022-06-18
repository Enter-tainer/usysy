use crate::{
  error::{Error, Result},
  parser::{get_text, to_source_span, useful_children},
};
use inkwell::module::Linkage;
use itertools::Itertools;
use miette::NamedSource;
use tree_sitter::Node;

use super::{BaseType, Generator, MBasicType};

impl<'ctx, 'node> Generator<'ctx, 'node> {
  fn generate_var(&mut self, is_const: bool, ty: BaseType<'node>, declarator: Node) -> Result<()> {
    let name = declarator.child_by_field_name("name").unwrap();

    let name_str = get_text(
      declarator.child_by_field_name("name").unwrap(),
      self.file.content,
    );
    if declarator.child_by_field_name("dimension").is_some() {
      todo!("array not supported");
    }
    if self.function_map.contains_key(name_str)
      || self.val_map_block_stack[0].contains_key(name_str)
    {
      return Err(Error::DuplicateGlobalSymbol {
        src: NamedSource::new(self.file.name, self.file.content.to_string()),
        range: to_source_span(name.range()),
      });
    }
    let llvm_type = ty.to_llvm_type(self.context);
    let global_value = self.module.add_global(llvm_type, None, name_str);
    global_value.set_linkage(Linkage::Common);
    if is_const {
      global_value.set_constant(true);
    }
    let init = declarator.child_by_field_name("init");
    if let Some(init) = init {
      let (expr_ty, val) = self.generate_expression(init)?;
      if expr_ty == BaseType::Int && ty == BaseType::Float {
        todo!("int -> float cast not supported");
      }
      global_value.set_initializer(&val);
    } else {
      global_value.set_initializer(&llvm_type.const_zero());
    }
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
    Ok(())
  }
  pub(super) fn generate_global_var(&mut self, root: Node) -> Result<()> {
    let ty: BaseType =
      get_text(root.child_by_field_name("type").unwrap(), self.file.content).try_into()?;
    let is_const = root.child_by_field_name("const").is_some();
    let declarators = {
      let mut cursor = root.walk();
      useful_children(&root, &mut cursor)
        .skip(1) // skip first ty
        .collect_vec()
    };
    for declarator in declarators {
      self.generate_var(is_const, ty.clone(), declarator)?;
    }
    Ok(())
  }
  pub(super) fn generate_local_var(&mut self, root: Node) -> Result<()> {
    Ok(())
  }
}
