use std::collections::HashMap;

use crate::{
  error::{Error, Result},
  parser::{get_text, to_source_span, useful_children},
};
use inkwell::{
  module::Linkage,
  types::{BasicMetadataTypeEnum, BasicType},
  values::BasicValue,
};
use itertools::Itertools;
use miette::NamedSource;
use tree_sitter::Node;

use super::{BaseType, Generator, MBasicType};

impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_function_proto(&mut self, function: Node) -> Result<()> {
    let ret_type = BaseType::try_from(get_text(
      function.child_by_field_name("return_type").unwrap(),
      self.file.content,
    ))?;
    let func_name = function.child_by_field_name("name").unwrap();
    let func_name_str = get_text(func_name, self.file.content);

    let params = function.child_by_field_name("param").unwrap();
    if self.function_map.contains_key(func_name_str)
      || self.val_map_block_stack[0].contains_key(func_name_str)
    {
      return Err(Error::DuplicateSymbol {
        src: NamedSource::new(self.file.name, self.file.content.to_string()),
        range: to_source_span(func_name.range()),
      });
    }
    let params = {
      let mut cursor = params.walk();
      useful_children(&params, &mut cursor)
        .map(|param| {
          let ty = param.child_by_field_name("type").unwrap();
          let ty = BaseType::try_from(get_text(ty, self.file.content));
          let name = param.child_by_field_name("name").unwrap();
          let name_str = get_text(name, self.file.content);
          if param.child_by_field_name("array").is_some() {
            todo!("array not supported!");
          }
          match ty {
            Ok(ty) => (ty, name_str),
            Err(_) => {
              todo!()
            }
          }
        })
        .collect_vec()
    };
    let llvm_params = params
      .iter()
      .map(|(param_type, _)| param_type.to_llvm_type(self.context))
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
          .map(|(param_type, name)| (name, MBasicType::new_with_base_mut(param_type)))
          .collect_vec(),
        false,
      ),
    );
    Ok(())
  }

  pub(super) fn generate_function_definition(&mut self, function: Node) -> Result<()> {
    let func_name = function.child_by_field_name("name").unwrap();
    let func_name_str = get_text(func_name, self.file.content);
    let func = self.module.get_function(func_name_str).unwrap();
    let param_list = function.child_by_field_name("param").unwrap();
    self.val_map_block_stack.push(HashMap::new());

    let (func_ty, func_params, _) = self.function_map.get(func_name_str).unwrap().to_owned();
    self.current_function = Some((func, func_ty.clone()));

    let func_block = self.context.append_basic_block(func, "entry");
    self.builder.position_at_end(func_block);

    let mut func_param_alloca = Vec::new();

    for ((name, ty), llvm_value) in func_params.into_iter().zip_eq(func.get_param_iter()) {
      llvm_value.set_name(name);
      let builder = self.context.create_builder();
      let func_entry = func.get_first_basic_block().unwrap();
      match func_entry.get_first_instruction() {
        Some(first_inst) => builder.position_before(&first_inst),
        None => builder.position_at_end(func_entry),
      }
      let llvm_type = ty.base_type.to_llvm_type(self.context);
      let alloca = builder.build_alloca(llvm_type, name);
      func_param_alloca.push(alloca);
      self.insert_to_val_map(&ty, name, alloca, param_list.range())?;
    }
    for (value, &param_ptr) in func.get_param_iter().zip_eq(func_param_alloca.iter()) {
      self.builder.build_store(param_ptr, value);
    }
    let stat = function.child_by_field_name("body").unwrap();
    self.generate_statement(stat)?;

    // let mut iter_block = func.get_first_basic_block();
    // while let Some(block) = iter_block {
    //   if block.get_terminator().is_none() {
    //     let terminator_builder = self.context.create_builder();
    //     terminator_builder.position_at_end(block);
    //     match func_ty.base_type {
    //       BaseType::Void => {
    //         terminator_builder.build_return(None);
    //       }
    //       _ => {
    //         let null_val = self.context.i32_type().const_zero();
    //         terminator_builder.build_return(Some(&null_val));
    //       }
    //     }
    //   }
    //   iter_block = block.get_next_basic_block();
    // }

    if !func.verify(true) {
      func.print_to_stderr();
      unreachable!();
    }

    self.val_map_block_stack.pop();
    self.current_function = None;
    Ok(())
  }
  // /* Input & output functions */
  // int getint(),getch(),getarray(int a[]);
  // float getfloat();
  // int getfarray(float a[]);
  //
  // void putint(int a),putch(int a),putarray(int n,int a[]);
  // void putfloat(float a);
  // void putfarray(int n, float a[]);
  //
  // void putf(char a[], ...);
  pub(super) fn generate_builtin_function(&mut self) -> Result<()> {
    let functions = [
      (
        "getint",
        (
          MBasicType {
            is_const: false,
            base_type: BaseType::Int,
          },
          Vec::new(),
          false,
        ),
      ),
      (
        "getch",
        (
          MBasicType {
            is_const: false,
            base_type: BaseType::Int,
          },
          Vec::new(),
          false,
        ),
      ),
      // (
      //   "getarray",
      //   (
      //     MBasicType {
      //       is_const: false,
      //       base_type: BaseType::Int,
      //     },
      //     vec![(
      //       "input_array",
      //       MBasicType {
      //         is_const: false,
      //         base_type: BaseType::Array(Box::new(BaseType::Int), vec![]),
      //       },
      //     )],
      //     false,
      //   ),
      // ),
      (
        "getfloat",
        (
          MBasicType {
            is_const: false,
            base_type: BaseType::Float,
          },
          Vec::new(),
          false,
        ),
      ),
      // (
      //   "getfarray",
      //   (
      //     MBasicType {
      //       is_const: false,
      //       base_type: BaseType::Int,
      //     },
      //     vec![(
      //       "input_array",
      //       MBasicType {
      //         is_const: false,
      //         base_type: BaseType::Array(Box::new(BaseType::Float), vec![]),
      //       },
      //     )],
      //     false,
      //   ),
      // ),
      (
        "putint",
        (
          MBasicType {
            is_const: false,
            base_type: BaseType::Void,
          },
          vec![(
            "output_int",
            MBasicType {
              is_const: false,
              base_type: BaseType::Int,
            },
          )],
          false,
        ),
      ),
      (
        "putch",
        (
          MBasicType {
            is_const: false,
            base_type: BaseType::Void,
          },
          vec![(
            "output_ch",
            MBasicType {
              is_const: false,
              base_type: BaseType::Int,
            },
          )],
          false,
        ),
      ),
      (
        "putfloat",
        (
          MBasicType {
            is_const: false,
            base_type: BaseType::Void,
          },
          vec![(
            "output_float",
            MBasicType {
              is_const: false,
              base_type: BaseType::Float,
            },
          )],
          false,
        ),
      ),
    ];
    for func @ (name, (ret_ty, params, is_va_arg)) in &functions {
      self.function_map.insert(func.0.to_string(), func.1.clone());
      let llvm_params = params
        .iter()
        .map(|(_, param_type)| param_type.base_type.to_llvm_type(self.context))
        .collect_vec();
      let meta_params = llvm_params
        .iter()
        .map(|ty| BasicMetadataTypeEnum::from(*ty))
        .collect::<Vec<BasicMetadataTypeEnum>>();
      let fn_ty = ret_ty
        .base_type
        .to_llvm_type(self.context)
        // TODO: va arg
        .fn_type(&meta_params, *is_va_arg);
      self
        .module
        .add_function(name, fn_ty, Some(Linkage::External));
    }
    Ok(())
  }
}
