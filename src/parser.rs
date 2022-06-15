use crate::error::{Error, Result};
use colored::*;
use tree_sitter::{Node, Parser, Tree};
pub fn parse(input: &str) -> Result<Tree> {
  let mut parser = Parser::new();
  let language = tree_sitter_c::language();
  parser.set_language(language)?;
  let tree = parser
    .parse(input.as_bytes(), None)
    .ok_or(Error::TreesitterParseFailed)?;
  Ok(tree)
}
#[allow(dead_code)]
fn dump_node_internal(node: &Node, level: usize, content: &str) {
  let node_text = node.utf8_text(content.as_bytes()).unwrap();
  println!(
    "{}{:?}{}",
    " ".repeat(level),
    node,
    if node.child_count() == 0 || !node_text.contains('\n') {
      format!(
        " {} {}",
        "->".cyan(),
        node_text.bold()
      )
      .bold()
    } else {
      "".to_owned().normal()
    }
  );
  let mut cursor = node.walk();
  if cursor.goto_first_child() {
    loop {
      dump_node_internal(&cursor.node(), level + 2, content);
      if !cursor.goto_next_sibling() {
        break;
      }
    }
  }
}

#[allow(dead_code)]
pub fn dump_node(node: &Node, content: &str) {
  dump_node_internal(node, 0, content);
}
