use super::Generator;

impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub fn print_function_proto(&self) {
    for (name, (return_type, params, va_arg)) in &self.function_map {
      let mut param_str = String::from("(");
      for j in params {
        param_str.push_str(format!("{j}, ").as_str());
      }
      if *va_arg {
        param_str.push_str("...");
      }
      param_str.push(')');
      println!("{name} {param_str} -> {return_type}",);
    }
  }
}
