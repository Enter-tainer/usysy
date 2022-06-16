mod global;
mod function;
use std::{
  collections::{HashMap, VecDeque},
  path::Path,
};

use hash_chain::ChainMap;
use inkwell::{
  basic_block::BasicBlock,
  builder::Builder,
  context::Context,
  module::Module,
  values::{FunctionValue, PointerValue},
};
use tree_sitter::Tree;

use crate::error::{Result, Error};

pub struct Generator<'ctx> {
  file: File<'ctx>,
  context: &'ctx Context,
  module: Module<'ctx>,
  builder: Builder<'ctx>,

  //>>>>>>>>>>>>>>>>>>>>>>>>
  //      LLVM Blocks
  //<<<<<<<<<<<<<<<<<<<<<<<<

  // value -> (type, pointer) map in a LLVM basic block
  val_map_block_stack: ChainMap<String, (BasicType, PointerValue<'ctx>)>,
  // current function block
  current_function: Option<(FunctionValue<'ctx>, BasicType)>,
  // break labels (in loop statements)
  break_labels: VecDeque<BasicBlock<'ctx>>,
  // continue labels (in loop statements)
  continue_labels: VecDeque<BasicBlock<'ctx>>,
  // hashset for functions
  function_map: HashMap<String, (BasicType, Vec<BasicType>, bool)>,
  // hashset for global variable
  global_variable_map: HashMap<String, (BasicType, PointerValue<'ctx>)>,
}
#[derive(Debug)]
pub struct BasicType {
  pub is_const: bool,
  pub base_type: BaseType,
}

#[derive(Debug)]
pub struct File<'ctx> {
  content: &'ctx str,
  name: &'ctx str,
}

#[derive(Debug)]
pub enum BaseType {
  Int,
  Float,
  Void,
}

impl TryFrom<&str> for BaseType {
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

impl<'ctx> Generator<'ctx> {
  pub fn new(context: &'ctx Context, path: &'ctx str, content: &'ctx str) -> Generator<'ctx> {
    let module_name = Path::new(path).file_stem().unwrap().to_str().unwrap();
    let file = File {
      content,
      name: path,
    };
    let module = context.create_module(module_name);
    let builder = context.create_builder();
    let global_variable_map = HashMap::new();
    let val_map_block_stack = ChainMap::new(global_variable_map);
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
      global_variable_map: HashMap::new(),
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
