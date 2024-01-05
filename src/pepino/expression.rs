use std::fmt::{Display, Formatter};

use crate::functions::Function;
use crate::generators::Generator;
use crate::operators::Operator;
use crate::units::Unit;
use crate::ComputeError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumericResult {
    pub value: f64,
    pub unit: Option<Unit>,
}

impl NumericResult {
    pub fn new(value: f64, unit: Option<Unit>) -> NumericResult {
        NumericResult { value, unit }
    }

    pub fn convert_to(self, to: Unit) -> Result<NumericResult, ComputeError> {
        let Some(unit) = self.unit else {
            return Ok(self);
        };

        if unit == to {
            return Ok(self);
        }

        let Some(v) = unit.conversion(self.value, &to) else {
            return Err(ComputeError::UnitConversionError(
                self.value,
                unit.to_string(),
                to.to_string(),
            ));
        };

        Ok(NumericResult::new(v, Some(to)))
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionToken {
    Operator(Operator),
    Function(Function),
    Generator(Generator),
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
    pub(super) fn new() -> Expression {
        Expression { tokens: vec![] }
    }

    pub(super) fn from_tokens(tokens: Vec<ExpressionToken>) -> Expression {
        Expression { tokens }
    }

    pub(super) fn push(&mut self, token: ExpressionToken) {
        self.tokens.push(token)
    }

    pub fn explain(&self) -> String {
        self.to_string()
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
