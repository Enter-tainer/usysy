extern crate libtest_mimic;

use inkwell::context::Context;
use itertools::Itertools;
use libtest_mimic::{run_tests, Arguments, Outcome, Test};
use sysy::{codegen::Generator, parser::parse, util::get_bc_exe_path};

use std::{
  env,
  error::Error,
  ffi::OsStr,
  fs,
  io::Write,
  path::{Path, PathBuf},
  process::{Command, Stdio},
};

fn main() {
  let args = Arguments::from_args();

  let tests = collect_tests();
  run_tests(&args, tests, run_test).exit();
}

/// Creates one test for each `.rs` file in the current directory or
/// sub-directories of the current directory.
fn collect_tests() -> Vec<Test<PathBuf>> {
  fn visit_dir(path: &Path, tests: &mut Vec<Test<PathBuf>>) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(path)? {
      let entry = entry?;
      let file_type = entry.file_type()?;

      // Handle files
      let path = entry.path();
      if file_type.is_file() {
        if path.extension() == Some(OsStr::new("sy")) {
          let name = path
            .strip_prefix(env::current_dir()?)?
            .display()
            .to_string();

          tests.push(Test {
            name,
            kind: "sysy".into(),
            is_ignored: false,
            is_bench: false,
            data: path,
          })
        }
      } else if file_type.is_dir() {
        // Handle directories
        visit_dir(&path, tests)?;
      }
    }

    Ok(())
  }

  // We recursively look for `.rs` files, starting from the current
  // directory.
  let mut tests = Vec::new();
  let mut current_dir = env::current_dir().expect("invalid working directory");
  current_dir.extend(["compiler2022", "runtime", "functional"]);
  visit_dir(&current_dir, &mut tests).expect("unexpected IO error");

  tests
}

/// Performs a couple of tidy tests.
fn run_test(test: &Test<PathBuf>) -> Outcome {
  let res = std::panic::catch_unwind(|| {
    let path = &test.data;
    let input_file = {
      let mut tmp = path.clone();
      tmp.set_extension("in");
      if tmp.exists() {
        let content = std::fs::read_to_string(tmp).unwrap();
        Some(content)
      } else {
        None
      }
    };
    let expected_out_path = {
      let mut tmp = path.clone();
      tmp.set_extension("out");
      tmp
    };
    let expected_output = std::fs::read_to_string(expected_out_path).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let tree = parse(&input).unwrap();
    let ctx = Context::create();
    let mut gen = Generator::new(&ctx, &input, &input);
    gen.gen(&tree).unwrap();
    let base = Path::new(&path);
    let (bc_path, exe_path) = get_bc_exe_path(base);
    gen.write(&bc_path);
    Command::new("llvm-dis").arg(&bc_path).output().unwrap();
    Command::new("clang")
      .args([
        &bc_path,
        "./compiler2022/runtime/sylib.c",
        &format!("-o{}", exe_path),
      ])
      .output()
      .unwrap();
    let run_cmd = {
      let mut cmd = Command::new(exe_path);
      if let Some(input) = input_file {
        let mut proc = cmd
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          .spawn()
          .unwrap();
        let mut stdin = proc.stdin.take().unwrap();
        stdin.write_all(input.as_bytes()).unwrap();
        drop(stdin);
        proc.wait_with_output().unwrap()
      } else {
        cmd.output().unwrap()
      }
    };
    let stdout = run_cmd.stdout;
    let stdout = String::from_utf8(stdout).unwrap();
    let ret_code = run_cmd.status.code().unwrap();
    let actual_output = format!("{}\n{}", stdout.trim(), ret_code);
    let actual_output = actual_output.lines().map(|l| l.trim()).join("\n");
    let expected_output = expected_output.lines().map(|l| l.trim()).join("\n");
    assert_eq!(expected_output.trim(), actual_output.trim());
  });
  match res {
    Ok(_) => Outcome::Passed,
    Err(_) => Outcome::Failed {
      msg: Some(format!("{} failed!", test.data.display())),
    },
  }
}
