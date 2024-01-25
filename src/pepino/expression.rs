use itertools::Itertools;
use std::fmt::{Display, Formatter};

use crate::functions::Function;
use crate::generators::Generator;
use crate::operators::{Operator, CONVERSION_CHARACTER};
use crate::units::Unit;
use crate::{string, ComputeError, Decimal};

#[derive(Debug, Clone, PartialEq)]
pub enum NumericExpression {
    Number(Decimal),
    NumberWithUnit(Decimal, Unit),
    MultipleNumbersWithUnit(Vec<(Decimal, Option<Unit>)>),
}

impl NumericExpression {
    pub fn with_unit(n: Decimal, unit: Option<Unit>) -> NumericExpression {
        match unit {
            None => NumericExpression::Number(n),
            Some(u) => NumericExpression::NumberWithUnit(n, u),
        }
    }

    pub fn with_multiple_units(values: Vec<(Decimal, Option<Unit>)>) -> NumericExpression {
        match values.len() {
            1 => NumericExpression::with_unit(values[0].0, values[0].1),
            _ => NumericExpression::MultipleNumbersWithUnit(values),
        }
    }

    pub fn with_units(n: Decimal, units: Vec<Unit>) -> NumericExpression {
        match units.len() {
            0 => NumericExpression::Number(n),
            1 => NumericExpression::NumberWithUnit(n, units[0]),
            _ => NumericExpression::MultipleNumbersWithUnit(
                units.iter().map(|u| (n, Some(*u))).collect(),
            ),
        }
    }

    pub fn values(&self) -> Vec<(Decimal, Option<Unit>)> {
        match self {
            NumericExpression::Number(n) => vec![(*n, None)],
            NumericExpression::NumberWithUnit(n, u) => vec![(*n, Some(*u))],
            NumericExpression::MultipleNumbersWithUnit(v) => v.clone(),
        }
    }

    pub fn units(&self) -> Vec<Option<Unit>> {
        self.values().iter().map(|(_, u)| *u).collect()
    }

    pub fn convert_to(
        &self,
        to: &Unit,
        force_unit: bool,
    ) -> Result<NumericExpression, ComputeError> {
        match self {
            NumericExpression::Number(n) => {
                return Ok(if force_unit {
                    NumericExpression::with_unit(*n, Some(*to))
                } else {
                    NumericExpression::with_unit(*n, None)
                })
            }
            NumericExpression::NumberWithUnit(n, u) => {
                if *u == *to {
                    return Ok(self.clone());
                }

                let Some(v) = u.conversion(n, &to) else {
                    return Err(ComputeError::UnitConversionError(
                        *n,
                        u.to_string_with_plural(&n),
                        to.to_string_with_plural(&n),
                    ));
                };

                Ok(NumericExpression::with_unit(v, Some(*to)))
            }
            NumericExpression::MultipleNumbersWithUnit(values) => {
                // it is ok to convert only one of the values
                let mut res = Vec::new();

                for (n, u) in values {
                    if let Some(u) = u {
                        if let Some(c) = u.conversion(n, to) {
                            res.push((c, Some(*to)))
                        };
                    } else {
                        res.push((*n, *u));
                    }
                }

                if res.is_empty() {
                    Err(ComputeError::OperatorsConversionError(
                        vec![self.clone()],
                        vec![values.iter().map(|v| v.1).collect()],
                    ))
                } else if res.len() == 1 {
                    Ok(NumericExpression::with_unit(res[0].0, res[0].1))
                } else {
                    Ok(NumericExpression::with_multiple_units(res))
                }
            }
        }
    }
}

impl Display for NumericExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericExpression::Number(n) => {
                write!(f, "{}", n)
            }
            NumericExpression::NumberWithUnit(n, u) => {
                write!(f, "{}{}", n, u.to_string_with_plural(n))
            }
            NumericExpression::MultipleNumbersWithUnit(values) => {
                let values = values
                    .iter()
                    .map(|v| {
                        format!(
                            "{}{}",
                            v.0,
                            if let Some(u) = v.1 {
                                u.to_string_with_plural(&v.0)
                            } else {
                                string!("")
                            }
                        )
                    })
                    .unique()
                    .join("|");
                write!(f, "{}", values)
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
    ConversionChain(Vec<Vec<Unit>>), // vector of unit conversions
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
                            unit.iter()
                                .map(|u| u.to_string_with_plural(&Decimal::ZERO))
                                .unique()
                                .join("|")
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
