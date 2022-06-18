use super::Generator;

impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub fn print_function_proto(&self) {
    for (name, (return_type, params, va_arg)) in &self.function_map {
      let mut param_str = String::from("(");
      for (name, ty) in params {
        param_str.push_str(format!("{name}: {ty} ").as_str());
      }
      if *va_arg {
        param_str.push_str("...");
      }
      param_str.push(')');
      println!("{name} {param_str} -> {return_type}",);
    }
  }
  pub fn print_global_var(&self) {
    for (name, (ty, _)) in &self.val_map_block_stack[0] {
      println!("{name}: {ty}");
    }
  }
}
