use std::collections::HashMap;

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
fn dump_node_internal(
  node: &Node,
  prefix: &str,
  content: &str,
  field_name: Option<&str>,
  is_last: bool,
  is_init: bool,
) {
  let node_text = node.utf8_text(content.as_bytes()).unwrap();
  let start = node.start_position();
  let end = node.end_position();
  let kind = node.kind();
  println!(
    "{}{}{}: `{}` {} - {}{}",
    prefix,
    if is_init {
      ""
    } else if is_last {
      "└──"
    } else {
      "├──"
    },
    match field_name {
      Some(name) => name.bold().yellow(),
      None => "[ANON]".normal(),
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
  let node_to_idx: HashMap<_, _> = {
    let mut cursor = node.walk();
    node
      .children(&mut cursor)
      .enumerate()
      .map(|(x, y)| (y, x))
      .collect()
  };
  let nodes: Vec<_> = {
    let mut cursor = node.walk();
    node.named_children(&mut cursor).collect_vec()
  };
  let prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });
  for i in nodes.into_iter().with_position() {
    match i {
      itertools::Position::First(n) | itertools::Position::Middle(n) => {
        dump_node_internal(
          &n,
          &prefix,
          content,
          node.field_name_for_child(node_to_idx[&n] as u32),
          false,
          false,
        );
      }
      itertools::Position::Last(n) | itertools::Position::Only(n) => {
        dump_node_internal(
          &n,
          &prefix,
          content,
          node.field_name_for_child(node_to_idx[&n] as u32),
          true,
          false,
        );
      }
    }
  }
}

#[allow(dead_code)]
pub fn dump_node(node: &Node, content: &str) {
  dump_node_internal(node, "", content, None, true, true);
}
