use crate::{
  error::{Error, Result},
  parser::{get_text, to_source_span, useful_children},
};
use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use itertools::Itertools;
use miette::NamedSource;
use tree_sitter::Node;

use super::{BaseType, Generator, MBasicType};

impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_function_proto(&mut self, function: Node) -> Result<()> {
    let ret_type = BaseType::try_from(get_text(
      function.child_by_field_name("type").unwrap(),
      self.file.content,
    ))?;
    let (func_name, params) = {
      let tmp = function.child_by_field_name("declarator").unwrap();
      (
        tmp.child_by_field_name("declarator").unwrap(),
        tmp.child_by_field_name("parameters").unwrap(),
      )
    };
    let func_name_str = get_text(func_name, self.file.content);
    if self.function_map.contains_key(func_name_str)
      || self.val_map_block_stack.has_at(0, func_name_str)
    {
      return Err(Error::DuplicateGlobalSymbol {
        src: NamedSource::new(self.file.name, self.file.content.to_string()),
        range: to_source_span(func_name.range()),
      });
    }
    let params = {
      let mut cursor = params.walk();
      useful_children(&params, &mut cursor)
        .map(|node| {
          let ty = BaseType::try_from(get_text(node, self.file.content));
          match ty {
            Ok(ty) => ty,
            Err(_) => {
              todo!()
            }
          }
        })
        .collect_vec()
    };
    let llvm_params = params
      .iter()
      .map(|param| param.to_llvm_type(self.context))
      .collect_vec();
    let meta_params = llvm_params
      .iter()
      .map(|ty| BasicMetadataTypeEnum::from(*ty))
      .collect::<Vec<BasicMetadataTypeEnum>>();
    let fn_ty = ret_type
      .to_llvm_type(self.context)
      // TODO: va arg
      .fn_type(&meta_params, false);
    self.module.add_function(func_name_str, fn_ty, None);
    self.function_map.insert(
      func_name_str.to_string(),
      (
        MBasicType::new_with_base_mut(ret_type),
        params
          .into_iter()
          .map(MBasicType::new_with_base_mut)
          .collect_vec(),
        false,
      ),
    );
    Ok(())
  }

  pub(super) fn generate_function_definition(&mut self, function: Node) -> Result<()> {
    todo!()
  }
}
