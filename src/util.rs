use std::{path::Path, process::Command};

pub fn get_bc_exe_path(base: &Path) -> (String, String) {
  let bc_path = format!("{}.bc", base.file_stem().unwrap().to_str().unwrap());
  let exe_path = format!("./{}.exe", base.file_stem().unwrap().to_str().unwrap());
  (bc_path, exe_path)
}

pub fn compile_with_clang(bc_path: &str, exe_path: &str) {
  Command::new("clang")
    .args([
      bc_path,
      "./compiler2022/runtime/sylib.c",
      &format!("-o{}", exe_path),
    ])
    .spawn()
    .unwrap();
}
