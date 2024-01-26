use std::collections::HashMap;
use std::sync::OnceLock;

use rust_decimal::MathematicalOps;
use strum_macros::Display;

use crate::{ComputeError, Decimal};

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
    pub unary_action: fn(right: Decimal) -> Result<Decimal, ComputeError>,
    pub binary_action: fn(left: Decimal, right: Decimal) -> Result<Decimal, ComputeError>,
}

pub(super) const CONVERSION_CHARACTER: char = '→';

pub fn operators() -> &'static HashMap<char, Operator> {
    static MEM: OnceLock<HashMap<char, Operator>> = OnceLock::new();
    MEM.get_or_init(|| {
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
                unary_action: unsupported_unary_operator,
                binary_action: |x, y| Ok(x * y),
            },
            Operator {
                representation: '/',
                priority: Priority::High,
                unary_action: unsupported_unary_operator,
                binary_action: |x, y| Ok(x / y),
            },
            Operator {
                representation: '^',
                priority: Priority::Highest,
                unary_action: unsupported_unary_operator,
                binary_action: |x, y| Ok(x.powd(y)),
            },
        ] {
            operators.insert(operator.representation, operator);
        }

        operators
    })
}

fn unsupported_unary_operator(_: Decimal) -> Result<Decimal, ComputeError> {
    Err(ComputeError::UnsupportedUnaryOperator)
}
