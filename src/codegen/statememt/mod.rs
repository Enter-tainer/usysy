mod assignment;
mod compound;
mod expression;
mod if_statement;
mod while_statement;

use crate::{
  error::{Error, Result},
  parser::to_source_span,
};
use miette::NamedSource;
use tree_sitter::Node;

use super::{BaseType, Generator};
impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_statement(&mut self, root: Node) -> Result<()> {
    let stat_type = root.kind();
    match stat_type {
      "compound_statement" => self.generate_compound_statement(root)?,
      "expression_statement" => self.generate_expression_statement(root)?,
      "if_statement" => self.generate_if_statement(root)?,
      "while_statement" => self.generate_while_statement(root)?,
      "assignment" => self.generate_assignment_statement(root)?,
      "declaration" => self.generate_local_var(root)?,
      "break_statement" => {
        let break_target = self.break_labels.back().ok_or(Error::KeywordNotInLoop {
          src: NamedSource::new(self.file.name, self.file.content.to_string()),
          range: to_source_span(root.range()),
        })?;
        self.builder.build_unconditional_branch(*break_target);
      }
      "continue_statement" => {
        let continue_target = self.continue_labels.back().ok_or(Error::KeywordNotInLoop {
          src: NamedSource::new(self.file.name, self.file.content.to_string()),
          range: to_source_span(root.range()),
        })?;
        self.builder.build_unconditional_branch(*continue_target);
      }
      "return_statement" => {
        let return_val = root.child_by_field_name("return_value");
        if let Some(return_val) = return_val {
          let (ty, val) = self.generate_expression(return_val)?;
          self.builder.build_return(Some(&val));
        } else {
          self.builder.build_return(None);
        };
      }
      _ => unreachable!("unknown statement type {stat_type}"),
    }
    Ok(())
  }
}
