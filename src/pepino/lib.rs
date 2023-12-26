use thiserror::Error;

pub use self::calc::Calc;

mod calc;
mod compute;
mod constants;
mod expression;
mod functions;
mod generators;
mod operators;
mod parser;
mod units;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Unbalanced parentheses")]
    UnbalancedParenthesesError(),
    #[error("Empty expression")]
    EmptyExpression,
    #[error("Invalid expression token: '{0}")]
    InvalidExpressionToken(String),
    #[error("Invalid expression")]
    InvalidExpression,
    #[error("Unsupported operation")]
    UnsupportedOperation,
}
