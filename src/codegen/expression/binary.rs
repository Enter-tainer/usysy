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
    if op == "||" || op == "&&" {
      Ok(match op {
        "||" => match lhs_t {
          // A || B ==>
          // if (A == 1) true else B
          BaseType::Int => {
            let current_fn = self.current_function.as_ref().unwrap().0;

            let lhs_i32 = self.builder.build_int_cast(
              lhs_v.into_int_value(),
              self.context.i32_type(),
              "lhs_i32",
            );
            let lhs_ne_0 = self.builder.build_int_compare(
              IntPredicate::NE,
              lhs_i32,
              self.context.i32_type().const_zero(),
              "i32ne0",
            );
            let lhs_ne_0_i1 =
              self
                .builder
                .build_int_cast(lhs_ne_0, self.context.bool_type(), "or_lhs_to_i1");
            let conseq_block = self
              .context
              .append_basic_block(current_fn, "short_circulated");
            let full_block = self
              .context
              .append_basic_block(current_fn, "not_circulated");
            let after_block = self
              .context
              .append_basic_block(current_fn, "after_or_block");
            let res = self
              .builder
              .build_alloca(self.context.bool_type(), "or_op_res");

            self
              .builder
              .build_conditional_branch(lhs_ne_0_i1, conseq_block, full_block);

            self.builder.position_at_end(conseq_block);
            self.builder.build_store(res, lhs_ne_0_i1);
            self.builder.build_unconditional_branch(after_block);

            self.builder.position_at_end(full_block);
            let (_rhs_t, rhs_v) = self.generate_expression(right)?;

            let rhs_i32 = self.builder.build_int_cast(
              rhs_v.into_int_value(),
              self.context.i32_type(),
              "rhs_i32",
            );
            let rhs_ne_0 = self.builder.build_int_compare(
              IntPredicate::NE,
              rhs_i32,
              self.context.i32_type().const_zero(),
              "i32ne0",
            );
            let rhs_ne_0_i1 =
              self
                .builder
                .build_int_cast(rhs_ne_0, self.context.bool_type(), "or_rhs_to_i1");
            let res_or = self.builder.build_or(lhs_ne_0_i1, rhs_ne_0_i1, "or");
            self.builder.build_store(res, res_or);
            self.builder.build_unconditional_branch(after_block);
            self.builder.position_at_end(after_block);
            let lor_res = self.builder.build_load(res, "load_lor_res");
            (BaseType::Int, lor_res)
          }
          t => {
            unreachable!("{op} is invalid for {t}")
          }
        },
        "&&" => match lhs_t {
          // A && B ==>
          // if(A == true) B else false
          BaseType::Int => {
            let current_fn = self.current_function.as_ref().unwrap().0;

            let lhs_i32 = self.builder.build_int_cast(
              lhs_v.into_int_value(),
              self.context.i32_type(),
              "lhs_i32",
            );
            let lhs_ne_0 = self.builder.build_int_compare(
              IntPredicate::NE,
              lhs_i32,
              self.context.i32_type().const_zero(),
              "i32ne0",
            );
            let lhs_ne_0_i1 =
              self
                .builder
                .build_int_cast(lhs_ne_0, self.context.bool_type(), "or_lhs_to_i1");
            let conseq_block = self
              .context
              .append_basic_block(current_fn, "short_circulated");
            let full_block = self
              .context
              .append_basic_block(current_fn, "not_circulated");
            let after_block = self
              .context
              .append_basic_block(current_fn, "after_or_block");
            let res = self
              .builder
              .build_alloca(self.context.bool_type(), "or_op_res");

            self
              .builder
              .build_conditional_branch(lhs_ne_0_i1, full_block, conseq_block);

            self.builder.position_at_end(conseq_block);
            self.builder.build_store(res, lhs_ne_0_i1);
            self.builder.build_unconditional_branch(after_block);

            self.builder.position_at_end(full_block);
            let (_rhs_t, rhs_v) = self.generate_expression(right)?;
            let rhs_i32 = self.builder.build_int_cast(
              rhs_v.into_int_value(),
              self.context.i32_type(),
              "rhs_i32",
            );
            let rhs_ne_0 = self.builder.build_int_compare(
              IntPredicate::NE,
              rhs_i32,
              self.context.i32_type().const_zero(),
              "i32ne0",
            );
            let rhs_ne_0_i1 =
              self
                .builder
                .build_int_cast(rhs_ne_0, self.context.bool_type(), "or_rhs_to_i1");
            let res_and = self.builder.build_and(lhs_ne_0_i1, rhs_ne_0_i1, "and");
            self.builder.build_store(res, res_and);
            self.builder.build_unconditional_branch(after_block);
            self.builder.position_at_end(after_block);
            let land_res = self.builder.build_load(res, "load_land_res");
            (BaseType::Int, land_res)
          }
          t => {
            unreachable!("{op} is invalid for {t}")
          }
        },
        _ => {
          unreachable!()
        }
      })
    } else {
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
        "==" | "!=" | ">" | "<" | ">=" | "<=" => match lhs_t {
          BaseType::Int => {
            let lhs_i32 = self.builder.build_int_cast(
              lhs_v.into_int_value(),
              self.context.i32_type(),
              "lhs_i32",
            );
            let rhs_i32 = self.builder.build_int_cast(
              rhs_v.into_int_value(),
              self.context.i32_type(),
              "rhs_i32",
            );
            let res =
              self
                .builder
                .build_int_compare(INT_COMP_OP_MAP[op], lhs_i32, rhs_i32, "int_comp_op");

            let res = self
              .builder
              .build_int_z_extend(res, self.context.i32_type(), "comp_op_i32");
            (BaseType::Int, res.as_basic_value_enum())
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
}
