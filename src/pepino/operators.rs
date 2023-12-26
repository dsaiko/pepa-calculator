use std::collections::HashMap;

use anyhow::Result;
use once_cell::sync::Lazy;
use strum_macros::Display;

use crate::ParsingError;

#[derive(Debug, Clone, Display, Eq, PartialEq)]
pub enum Priority {
    Low,
    High,
    Highest,
}

#[derive(Debug, Clone)]
pub struct Operator {
    pub representation: char,
    pub priority: Priority,
    pub unary_action: fn(right: f64) -> Result<f64>,
    pub binary_action: fn(left: f64, right: f64) -> Result<f64>,
}

pub static OPERATORS: Lazy<HashMap<char, Operator>> = Lazy::new(|| {
    let mut operators = HashMap::new();

    for operator in [
        Operator {
            representation: '+',
            priority: Priority::Low,
            unary_action: Ok,
            binary_action: |x, y| Ok(x + y),
        },
        Operator {
            representation: '-',
            priority: Priority::Low,
            unary_action: |x| Ok(-x),
            binary_action: |x, y| Ok(x - y),
        },
        Operator {
            representation: '*',
            priority: Priority::High,
            unary_action: unsupported_operator,
            binary_action: |x, y| Ok(x * y),
        },
        Operator {
            representation: '/',
            priority: Priority::High,
            unary_action: unsupported_operator,
            binary_action: |x, y| Ok(x / y),
        },
        Operator {
            representation: '^',
            priority: Priority::Highest,
            unary_action: unsupported_operator,
            binary_action: |x, y| Ok(x.powf(y)),
        },
    ] {
        operators.insert(operator.representation, operator);
    }

    operators
});

fn unsupported_operator(_: f64) -> Result<f64> {
    Err(ParsingError::UnsupportedOperation.into())
}
