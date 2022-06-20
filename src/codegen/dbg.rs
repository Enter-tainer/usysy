use colored::Colorize;

use super::Generator;

impl<'ctx> Generator<'ctx> {
  pub fn print_function_proto(&self) {
    println!("{}", "function protos: ".bold());
    for (name, (return_type, params, va_arg)) in &self.function_map {
      let mut param_str = String::from("(");
      for (name, ty) in params {
        param_str.push_str(format!("{name}: {ty}, ").as_str());
      }
      if *va_arg {
        param_str.push_str("...");
      }
      param_str.push(')');
      println!("{name} {param_str} -> {return_type}",);
    }
  }
  pub fn print_global_var(&self) {
    println!("{}", "global vars: ".bold());
    for (name, (ty, _)) in &self.val_map_block_stack[0] {
      println!("{name}: {ty}");
    }
  }
}
