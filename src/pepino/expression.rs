use std::fmt::{Display, Formatter};

use crate::operators::Operator;
use crate::ComputedResult;

#[derive(Debug, Clone)]
pub enum ExpressionToken {
    Operator(&'static Operator),
    ComputedResult(ComputedResult),
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
                ExpressionToken::ComputedResult(r) => match r {
                    ComputedResult::Numeric(n, u) => match u {
                        None => write!(f, "{}", n)?,
                        Some(u) => write!(f, "{}{}", n, u)?,
                    },
                    ComputedResult::Variable(x) => write!(f, "{}", x)?,
                },
                ExpressionToken::Expression(e) => write!(f, "({})", e)?,
            }
        }

        Ok(())
    }
}
