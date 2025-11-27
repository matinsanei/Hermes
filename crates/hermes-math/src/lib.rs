#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Constant(f32),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    pub fn eval(&self, context: &impl Context) -> Result<f32, EvalError> {
        match self {
            Expr::Constant(val) => Ok(*val),
            Expr::Variable(name) => context.get_var(name).ok_or(EvalError::VariableNotFound(name.clone())),
            Expr::Add(lhs, rhs) => Ok(lhs.eval(context)? + rhs.eval(context)?),
            Expr::Sub(lhs, rhs) => Ok(lhs.eval(context)? - rhs.eval(context)?),
            Expr::Mul(lhs, rhs) => Ok(lhs.eval(context)? * rhs.eval(context)?),
            Expr::Div(lhs, rhs) => {
                let r = rhs.eval(context)?;
                if r == 0.0 {
                    Err(EvalError::DivisionByZero)
                } else {
                    Ok(lhs.eval(context)? / r)
                }
            }
            Expr::Neg(expr) => Ok(-expr.eval(context)?),
        }
    }
}

pub trait Context {
    fn get_var(&self, name: &str) -> Option<f32>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    VariableNotFound(String),
    DivisionByZero,
}

// Simple HashMap-based context for testing (requires std or hashbrown, but we can implement a simple vector based one for no_std)
pub struct SimpleContext {
    vars: Vec<(String, f32)>,
}

impl SimpleContext {
    pub fn new() -> Self {
        Self { vars: Vec::new() }
    }

    pub fn set(&mut self, name: &str, value: f32) {
        if let Some(idx) = self.vars.iter().position(|(n, _)| n == name) {
            self.vars[idx].1 = value;
        } else {
            self.vars.push((String::from(name), value));
        }
    }
}

impl Context for SimpleContext {
    fn get_var(&self, name: &str) -> Option<f32> {
        self.vars.iter().find(|(n, _)| n == name).map(|(_, v)| *v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let expr = Expr::Add(
            Box::new(Expr::Constant(10.0)),
            Box::new(Expr::Constant(20.0)),
        );
        let ctx = SimpleContext::new();
        assert_eq!(expr.eval(&ctx), Ok(30.0));
    }

    #[test]
    fn test_variable() {
        let expr = Expr::Mul(
            Box::new(Expr::Variable(String::from("x"))),
            Box::new(Expr::Constant(2.0)),
        );
        let mut ctx = SimpleContext::new();
        ctx.set("x", 5.0);
        assert_eq!(expr.eval(&ctx), Ok(10.0));
    }
}
