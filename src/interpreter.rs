use std::fmt;

use super::lexer::{Annot, Loc};
use super::parser::{print_annot, Ast, BinOp, UniOp};

pub type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn eval(&mut self, expr: &Ast) -> Result<i64> {
        use super::parser::AstNode::*;
        match expr.value {
            Num(n) => Ok(n as i64),
            UniOp { ref op, ref e } => {
                let e = self.eval(e)?;
                self.eval_uniop(op, e)
                    .map_err(|err| InterpreterError::new(err, expr.loc.clone()))
            }
            BinOp {
                ref op,
                ref l,
                ref r,
            } => {
                let l = self.eval(l)?;
                let r = self.eval(r)?;
                self.eval_binop(op, l, r)
                    .map_err(|err| InterpreterError::new(err, expr.loc.clone()))
            }
        }
    }

    fn eval_uniop(&mut self, op: &UniOp, n: i64) -> std::result::Result<i64, InterpreterErrorKind> {
        use super::parser::UniOpKind::*;
        match op.value {
            Plus => Ok(n),
            Minus => Ok(-n),
        }
    }

    fn eval_binop(
        &mut self,
        op: &BinOp,
        l: i64,
        r: i64,
    ) -> std::result::Result<i64, InterpreterErrorKind> {
        use super::parser::BinOpKind::*;
        match op.value {
            Add => Ok(l + r),
            Sub => Ok(l - r),
            Mul => Ok(l * r),
            Div => {
                if r == 0 {
                    Err(InterpreterErrorKind::DivisionByZero)
                } else {
                    Ok(l / r)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InterpreterErrorKind {
    DivisionByZero,
}

type InterpreterError = Annot<InterpreterErrorKind>;

impl InterpreterError {
    pub fn show_diagnostic(&self, input: &str) {
        use self::InterpreterErrorKind::*;
        let (err, loc): (&std::error::Error, &Loc) = match self.value {
            DivisionByZero => (self, &self.loc),
        };
        eprintln!("{}", err);
        print_annot(input, loc);
    }
}

impl std::error::Error for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "division by zero error")
    }
}
