use std::fmt::{Display, Formatter};

use crate::functions::Function;
use crate::generators::Generator;
use crate::operators::Operator;
use crate::units::Unit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumericResult {
    pub value: f64,
    pub unit: Option<Unit>,
}

impl NumericResult {
    pub fn new(value: f64, unit: Option<Unit>) -> NumericResult {
        NumericResult { value, unit }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionToken {
    Operator(&'static Operator),
    Function(&'static Function),
    Generator(&'static Generator),
    Numeric(NumericResult),
    List(Vec<Expression>),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub tokens: Vec<ExpressionToken>,
}

impl Default for Expression {
    fn default() -> Self {
        Expression::new()
    }
}

impl Expression {
    pub fn new() -> Expression {
        Expression { tokens: vec![] }
    }

    pub fn from_tokens(tokens: Vec<ExpressionToken>) -> Expression {
        Expression { tokens }
    }

    pub fn push(&mut self, token: ExpressionToken) {
        self.tokens.push(token)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for token in self.tokens.iter() {
            match token {
                ExpressionToken::Operator(o) => write!(f, "{}", o.representation)?,
                ExpressionToken::Numeric(n) => match n.unit {
                    None => write!(f, "{}", n.value)?,
                    Some(u) => write!(f, "{}{}", n.value, u)?,
                },
                ExpressionToken::Expression(e) => write!(f, "({})", e)?,
                ExpressionToken::Function(fce) => write!(f, "{}", fce.representation)?,
                ExpressionToken::Generator(g) => write!(f, "{}", g.fce_name)?,
                ExpressionToken::List(list) => {
                    write!(f, "(")?;
                    let mut first = true;
                    for e in list {
                        if !first {
                            write!(f, ",")?;
                        }
                        write!(f, "{}", e)?;
                        first = false;
                    }
                    write!(f, ")")?;
                }
            }
        }

        Ok(())
    }
}
