use crate::{
  error::{Error, Result},
  parser::{to_source_span},
};
use inkwell::{
  values::{
    BasicValueEnum,
    InstructionOpcode::{FPToSI, SIToFP},
    PointerValue,
  },
};

use miette::NamedSource;
use tree_sitter::{Range};

use super::{BaseType, Generator, MBasicType};

// pub(super) fn try_cast(src: MBasicType, dst: MBasicType)

impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn no_terminator(&self) -> bool {
    let block = self.builder.get_insert_block();
    let terminator = block.unwrap().get_terminator();
    terminator.is_none()
  }
  pub(super) fn insert_to_val_map(
    &mut self,
    var_type: &MBasicType<'node>,
    identifier: &str,
    ptr: PointerValue<'ctx>,
    range: Range,
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

  pub(super) fn get_in_value_map(
    &self,
    identifier: &str,
    range: Range,
  ) -> Result<(MBasicType, PointerValue<'ctx>)> {
    for map in self.val_map_block_stack.iter().rev() {
      if map.contains_key(identifier) {
        return Ok(map[identifier].clone());
      }
    }
    Err(Error::VariableNotFound {
      src: NamedSource::new(self.file.name, self.file.content.to_string()),
      range: to_source_span(range),
    })
  }

  pub(crate) fn cast_value(
    &self,
    curr_type: &BaseType,
    curr_val: &BasicValueEnum<'ctx>,
    dest_type: &BaseType,
    range: Range,
  ) -> Result<BasicValueEnum<'ctx>> {
    if curr_type == dest_type {
      return Ok(curr_val.to_owned());
    }

    let llvm_type = dest_type.to_llvm_type(self.context);

    Ok(self.builder.build_cast(
      match (curr_type, dest_type) {
        (BaseType::Int, BaseType::Float) => SIToFP,
        (BaseType::Float, BaseType::Int) => FPToSI,
        _ => {
          return Err(Error::InvalidCast {
            src: NamedSource::new(self.file.name, self.file.content.to_string()),
            range: to_source_span(range),
          })
        }
      },
      *curr_val,
      llvm_type,
      "cast",
    ))
  }
}
