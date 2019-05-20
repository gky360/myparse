use super::parser::{Ast, BinOp, UniOp};

pub struct RpnCompiler;

impl RpnCompiler {
    pub fn new() -> Self {
        RpnCompiler
    }

    pub fn compile(&mut self, expr: &Ast) -> String {
        let mut buf = String::new();
        self.compile_inner(expr, &mut buf);
        buf
    }

    fn compile_inner(&mut self, expr: &Ast, buf: &mut String) {
        use super::parser::AstNode::*;
        match expr.value {
            Num(n) => buf.push_str(&n.to_string()),
            UniOp { ref op, ref e } => {
                self.compile_uniop(op, buf);
                self.compile_inner(e, buf);
            }
            BinOp {
                ref op,
                ref l,
                ref r,
            } => {
                self.compile_inner(l, buf);
                buf.push_str(" ");
                self.compile_inner(r, buf);
                buf.push_str(" ");
                self.compile_binop(op, buf);
            }
        }
    }

    fn compile_uniop(&mut self, op: &UniOp, buf: &mut String) {
        use super::parser::UniOpKind::*;
        match op.value {
            Plus => buf.push_str("+"),
            Minus => buf.push_str("-"),
        }
    }

    fn compile_binop(&mut self, op: &BinOp, buf: &mut String) {
        use super::parser::BinOpKind::*;
        match op.value {
            Add => buf.push_str("+"),
            Sub => buf.push_str("-"),
            Mul => buf.push_str("*"),
            Div => buf.push_str("/"),
        }
    }
}
