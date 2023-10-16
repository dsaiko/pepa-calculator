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
    pub unary_action: fn(left: f64) -> Result<f64>,
    pub binary_action: fn(left: f64, right: f64) -> Result<f64>,
}

pub(super) static OPERATORS: Lazy<HashMap<char, Operator>> = Lazy::new(|| {
    let mut operators = HashMap::new();

    for operator in [
        Operator {
            representation: '+',
            priority: Priority::Low,
            unary_action: plus_unary,
            binary_action: plus_binary,
        },
        Operator {
            representation: '-',
            priority: Priority::Low,
            unary_action: minus_unary,
            binary_action: minus_binary,
        },
        Operator {
            representation: '*',
            priority: Priority::High,
            unary_action: unsupported_unary,
            binary_action: multiply_binary,
        },
        Operator {
            representation: '/',
            priority: Priority::High,
            unary_action: unsupported_unary,
            binary_action: divide_binary,
        },
        Operator {
            representation: '^',
            priority: Priority::Highest,
            unary_action: unsupported_unary,
            binary_action: pow_binary,
        },
    ] {
        operators.insert(operator.representation, operator);
    }

    operators
});

fn unsupported_unary(_: f64) -> Result<f64> {
    Err(ParsingError::UnsupportedOperation.into())
}

fn plus_unary(left: f64) -> Result<f64> {
    Ok(left)
}

fn plus_binary(left: f64, right: f64) -> Result<f64> {
    Ok(left + right)
}

fn minus_unary(left: f64) -> Result<f64> {
    Ok(-left)
}

fn minus_binary(left: f64, right: f64) -> Result<f64> {
    Ok(left - right)
}

fn multiply_binary(left: f64, right: f64) -> Result<f64> {
    Ok(left * right)
}

fn divide_binary(left: f64, right: f64) -> Result<f64> {
    Ok(left / right)
}

fn pow_binary(left: f64, right: f64) -> Result<f64> {
    Ok(left.powf(right))
}
