use crate::error::{Error, Result};
use colored::*;
use itertools::Itertools;
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
fn dump_node_internal(node: &Node, prefix: &str, content: &str, is_last: bool, is_init: bool) {
  let node_text = node.utf8_text(content.as_bytes()).unwrap();
  let start = node.start_position();
  let end = node.end_position();
  let kind = node.kind();
  println!(
    "{}{}`{}` {} - {}{}",
    prefix,
    if is_init {
      ""
    } else if is_last {
      "└──"
    } else {
      "├──"
    },
    kind.bold(),
    start,
    end,
    if node.child_count() == 0 || !node_text.contains('\n') {
      format!(" {} {}", "->".cyan(), node_text.bold()).bold()
    } else {
      "".to_owned().normal()
    }
  );
  let mut cursor = node.walk();
  if cursor.goto_first_child() {
    let prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });
    let mut nodes = Vec::new();
    loop {
      nodes.push(cursor.node());
      if !cursor.goto_next_sibling() {
        break;
      }
    }
    for i in nodes.into_iter().with_position() {
      match i {
        itertools::Position::First(i) | itertools::Position::Middle(i) => {
          dump_node_internal(&i, &prefix, content, false, false);
        }
        itertools::Position::Last(i) | itertools::Position::Only(i) => {
          dump_node_internal(&i, &prefix, content, true, false);
        }
      }
    }
  }
}

#[allow(dead_code)]
pub fn dump_node(node: &Node, content: &str) {
  dump_node_internal(node, "", content, true, true);
}
