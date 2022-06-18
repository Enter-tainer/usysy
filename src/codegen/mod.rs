mod dbg;
mod function;
mod global;
mod var;
mod statememt;
mod expression;
mod utils;
use std::{
  collections::{HashMap, VecDeque},
  fmt::Display,
  path::Path,
};

use inkwell::{
  basic_block::BasicBlock,
  builder::Builder,
  context::Context,
  module::Module,
  types::{BasicType, BasicTypeEnum},
  values::{FunctionValue, PointerValue},
};
use tree_sitter::{Node, Tree};

use crate::error::{Error, Result};

pub struct Generator<'ctx, 'node> {
  file: File<'ctx>,
  context: &'ctx Context,
  module: Module<'ctx>,
  builder: Builder<'ctx>,

  //>>>>>>>>>>>>>>>>>>>>>>>>
  //      LLVM Blocks
  //<<<<<<<<<<<<<<<<<<<<<<<<

  // value -> (type, pointer) map in a LLVM basic block
  val_map_block_stack: Vec<HashMap<String, (MBasicType<'node>, PointerValue<'ctx>)>>,
  // current function block
  current_function: Option<(FunctionValue<'ctx>, MBasicType<'node>)>,
  // break labels (in loop statements)
  break_labels: VecDeque<BasicBlock<'ctx>>,
  // continue labels (in loop statements)
  continue_labels: VecDeque<BasicBlock<'ctx>>,
  // hashset for functions
  function_map: HashMap<String, (MBasicType<'node>, Vec<(&'ctx str, MBasicType<'node>)>, bool)>,
  // hashset for global variable
}
#[derive(Debug, Clone, PartialEq)]
pub struct MBasicType<'node> {
  pub is_const: bool,
  pub base_type: BaseType<'node>,
}

impl<'node> MBasicType<'node> {
  pub fn new_with_base(base_type: BaseType<'node>, is_const: bool) -> Self {
    Self {
      is_const,
      base_type,
    }
  }
  pub fn new_with_base_mut(base_type: BaseType<'node>) -> Self {
    Self {
      is_const: false,
      base_type,
    }
  }
}

impl<'node> Display for MBasicType<'node> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "{} {}",
      if self.is_const { "const" } else { "" },
      self.base_type
    ))
  }
}

#[derive(Debug)]
pub struct File<'ctx> {
  content: &'ctx str,
  name: &'ctx str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseType<'node> {
  Int,
  Float,
  Void,
  Array(
    /// element type
    Box<BaseType<'node>>,
    /// array length, from high-dimension to low-dimension
    Vec<Node<'node>>,
  ),
}

impl<'node> Display for BaseType<'node> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BaseType::Int => f.write_str("i32"),
      BaseType::Float => f.write_str("f32"),
      BaseType::Void => f.write_str("void"),
      BaseType::Array(b, _) => f.write_fmt(format_args!("{} array", b)),
    }
  }
}

impl<'node, 'ctx> BaseType<'node> {
  pub fn to_llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
    match self {
      BaseType::Int => ctx.i32_type().as_basic_type_enum(),
      BaseType::Float => ctx.f32_type().as_basic_type_enum(),
      BaseType::Void => ctx.i8_type().as_basic_type_enum(),
      BaseType::Array(_, _) => todo!(),
    }
  }
}

impl<'node> TryFrom<&str> for BaseType<'node> {
  type Error = Error;

  fn try_from(value: &str) -> Result<Self> {
    match value {
      "int" => Ok(BaseType::Int),
      "float" => Ok(BaseType::Float),
      "void" => Ok(BaseType::Void),
      _ => Err(Error::UnknownType()),
    }
  }
}

impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub fn new(
    context: &'ctx Context,
    path: &'ctx str,
    content: &'ctx str,
  ) -> Generator<'ctx, 'node> {
    let module_name = Path::new(path).file_stem().unwrap().to_str().unwrap();
    let file = File {
      content,
      name: path,
    };
    let module = context.create_module(module_name);
    let builder = context.create_builder();
    let global_variable_map = HashMap::new();
    let val_map_block_stack = vec![global_variable_map];
    Generator {
      file,
      context,
      module,
      builder,
      val_map_block_stack,
      current_function: None,
      break_labels: VecDeque::new(),
      continue_labels: VecDeque::new(),
      function_map: HashMap::new(),
    }
  }
  pub fn gen(&mut self, ast: &Tree) -> Result<()> {
    let root = ast.root_node();
    self.generate_global_proto(root)?;
    self.generate_global_definition(root)?;
    Ok(())
  }
  pub fn write(&self, path: &str) {
    self.module.write_bitcode_to_path(Path::new(path));
  }
}
