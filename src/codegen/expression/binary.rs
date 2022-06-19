use inkwell::{
  values::{BasicValue, BasicValueEnum},
  FloatPredicate, IntPredicate,
};
use phf::phf_map;
use tree_sitter::Node;

use super::{BaseType, Generator};
use crate::{error::Result, parser::get_text};

static INT_COMP_OP_MAP: phf::Map<&'static str, IntPredicate> = phf_map! {
    "==" => IntPredicate::EQ,
    "!=" => IntPredicate::NE,
    ">" => IntPredicate::SGT,
    "<" => IntPredicate::SLT,
    ">=" => IntPredicate::SGE,
    "<=" => IntPredicate::SLE,
};

static FLOAT_COMP_OP_MAP: phf::Map<&'static str, FloatPredicate> = phf_map! {
    "==" => FloatPredicate::OEQ,
    "!=" => FloatPredicate::ONE,
    ">" => FloatPredicate::OGT,
    "<" => FloatPredicate::OLT,
    ">=" => FloatPredicate::OGE,
    "<=" => FloatPredicate::OLE,
};

impl<'ctx, 'node> Generator<'ctx, 'node> {
  pub(super) fn generate_binary_expression(
    &self,
    root: Node,
  ) -> Result<(BaseType, BasicValueEnum<'ctx>)> {
    let left = root.child_by_field_name("left").unwrap();
    let op = get_text(
      root.child_by_field_name("operator").unwrap(),
      self.file.content,
    );
    let right = root.child_by_field_name("right").unwrap();
    let (lhs_t, lhs_v) = self.generate_expression(left)?;
    let (rhs_t, rhs_v) = self.generate_expression(right)?;
    assert_eq!(lhs_t, rhs_t);
    Ok(match op {
      "+" => match lhs_t {
        BaseType::Int => (
          lhs_t,
          self
            .builder
            .build_int_add(lhs_v.into_int_value(), rhs_v.into_int_value(), "iadd")
            .as_basic_value_enum(),
        ),
        BaseType::Float => (
          lhs_t,
          self
            .builder
            .build_float_add(lhs_v.into_float_value(), rhs_v.into_float_value(), "fadd")
            .as_basic_value_enum(),
        ),
        t => {
          unreachable!("{op} is invalid for {t}")
        }
      },
      "-" => match lhs_t {
        BaseType::Int => (
          lhs_t,
          self
            .builder
            .build_int_sub(lhs_v.into_int_value(), rhs_v.into_int_value(), "isub")
            .as_basic_value_enum(),
        ),
        BaseType::Float => (
          lhs_t,
          self
            .builder
            .build_float_sub(lhs_v.into_float_value(), rhs_v.into_float_value(), "fsub")
            .as_basic_value_enum(),
        ),
        t => {
          unreachable!("{op} is invalid for {t}")
        }
      },
      "*" => match lhs_t {
        BaseType::Int => (
          lhs_t,
          self
            .builder
            .build_int_mul(lhs_v.into_int_value(), rhs_v.into_int_value(), "imul")
            .as_basic_value_enum(),
        ),
        BaseType::Float => (
          lhs_t,
          self
            .builder
            .build_float_mul(lhs_v.into_float_value(), rhs_v.into_float_value(), "fmul")
            .as_basic_value_enum(),
        ),
        t => {
          unreachable!("{op} is invalid for {t}")
        }
      },
      "/" => match lhs_t {
        BaseType::Int => (
          lhs_t,
          self
            .builder
            .build_int_signed_div(lhs_v.into_int_value(), rhs_v.into_int_value(), "idiv")
            .as_basic_value_enum(),
        ),
        BaseType::Float => (
          lhs_t,
          self
            .builder
            .build_float_div(lhs_v.into_float_value(), rhs_v.into_float_value(), "fdiv")
            .as_basic_value_enum(),
        ),
        t => {
          unreachable!("{op} is invalid for {t}")
        }
      },
      "%" => match lhs_t {
        BaseType::Int => (
          lhs_t,
          self
            .builder
            .build_int_signed_rem(lhs_v.into_int_value(), rhs_v.into_int_value(), "idiv")
            .as_basic_value_enum(),
        ),
        t => {
          unreachable!("{op} is invalid for {t}")
        }
      },
      "||" => match lhs_t {
        BaseType::Int => {
          let lhs_ne_0 = self.builder.build_int_compare(
            IntPredicate::NE,
            lhs_v.into_int_value(),
            self.context.i32_type().const_zero(),
            "i32ne0",
          );
          let rhs_ne_0 = self.builder.build_int_compare(
            IntPredicate::NE,
            lhs_v.into_int_value(),
            self.context.i32_type().const_zero(),
            "i32ne0",
          );
          let res = self.builder.build_or(lhs_ne_0, rhs_ne_0, "or");
          (BaseType::Int, res.as_basic_value_enum())
        }
        t => {
          unreachable!("{op} is invalid for {t}")
        }
      },
      "&&" => match lhs_t {
        BaseType::Int => {
          let lhs_ne_0 = self.builder.build_int_compare(
            IntPredicate::NE,
            lhs_v.into_int_value(),
            self.context.i32_type().const_zero(),
            "i32ne0",
          );
          let rhs_ne_0 = self.builder.build_int_compare(
            IntPredicate::NE,
            lhs_v.into_int_value(),
            self.context.i32_type().const_zero(),
            "i32ne0",
          );
          let res = self.builder.build_and(lhs_ne_0, rhs_ne_0, "and");
          (BaseType::Int, res.as_basic_value_enum())
        }
        t => {
          unreachable!("{op} is invalid for {t}")
        }
      },
      "==" | "!=" | ">" | "<" | ">=" | "<=" => match lhs_t {
        BaseType::Int => {
          let res = self
            .builder
            .build_int_compare(
              INT_COMP_OP_MAP[op],
              lhs_v.into_int_value(),
              rhs_v.into_int_value(),
              "int_comp_op",
            )
            .as_basic_value_enum();
          (BaseType::Int, res)
        }
        BaseType::Float => {
          let res = self
            .builder
            .build_float_compare(
              FLOAT_COMP_OP_MAP[op],
              lhs_v.into_float_value(),
              rhs_v.into_float_value(),
              "float_comp_op",
            )
            .as_basic_value_enum();
          (BaseType::Int, res)
        }
        t => {
          unreachable!("{op} is invalid for {t}")
        }
      },
      op => unreachable!("invalid operator {op}"),
    })
  }
}
