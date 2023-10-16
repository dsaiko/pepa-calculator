use thiserror::Error;

pub use self::computer::ComputedResult;
pub use self::computer::Computer;

mod compute;
mod computer;
mod expression;
mod operators;
mod parser;
mod units;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Unbalanced parentheses")]
    UnbalancedParenthesesError(),
    #[error("Empty expression")]
    EmptyExpression,
    #[error("Invalid variable name: '{0}'")]
    InvalidVariableName(String),
    #[error("Invalid expression")]
    InvalidExpression,
    #[error("Unsupported operation")]
    UnsupportedOperation,
}
