use std::fmt::{Display, Formatter};

use crate::functions::Function;
use crate::generators::Generator;
use crate::operators::{Operator, CONVERSION_CHARACTER};
use crate::units::{Unit, UnitDefinition};
use crate::{ComputeError, Decimal};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericExpression {
    Decimal(Decimal),
    DecimalWithUnit(Decimal, Unit),
}

impl NumericExpression {
    pub fn new(value: Decimal, unit: Option<Unit>) -> NumericExpression {
        if let Some(unit) = unit {
            NumericExpression::DecimalWithUnit(value, unit)
        } else {
            NumericExpression::Decimal(value)
        }
    }

    pub fn value(&self) -> Decimal {
        match self {
            NumericExpression::Decimal(n) => *n,
            NumericExpression::DecimalWithUnit(n, _) => *n,
        }
    }

    pub fn unit(&self) -> Option<Unit> {
        match self {
            NumericExpression::Decimal(_) => None,
            NumericExpression::DecimalWithUnit(_, u) => Some(*u),
        }
    }

    pub fn convert_to(self, to: Unit, force_unit: bool) -> Result<NumericExpression, ComputeError> {
        match self {
            NumericExpression::Decimal(n) => {
                return Ok(if force_unit {
                    NumericExpression::new(n, Some(to))
                } else {
                    NumericExpression::new(n, None)
                })
            }
            NumericExpression::DecimalWithUnit(n, u) => {
                if u == to {
                    return Ok(self);
                }

                let Some(v) = u.conversion(n, &to) else {
                    return Err(ComputeError::UnitConversionError(
                        n,
                        u.to_string_with_plural(&n),
                        to.to_string_with_plural(&n),
                    ));
                };

                Ok(NumericExpression::new(v, Some(to)))
            }
        }
    }
}

impl Display for NumericExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericExpression::Decimal(n) => {
                write!(f, "{}", n)
            }
            NumericExpression::DecimalWithUnit(n, u) => {
                write!(f, "{}{}", n, u.to_string_with_plural(n))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionToken {
    Operator(Operator),
    Function(Function),
    Generator(Generator),
    Numeric(NumericExpression),
    List(Vec<Expression>),
    Expression(Expression),
    ConversionChain(Vec<UnitDefinition>), // vector of unit conversions
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
                ExpressionToken::Numeric(n) => write!(f, "{}", n)?,
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
                ExpressionToken::ConversionChain(units) => {
                    for unit in units {
                        write!(
                            f,
                            "{}{}",
                            CONVERSION_CHARACTER,
                            unit.to_string_with_plural(&Decimal::ZERO)
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
