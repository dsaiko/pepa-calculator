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

#[derive(Error, Debug, Clone)]
pub enum ParserError {
    #[error("Unbalanced parentheses: '{0}'")]
    UnbalancedParentheses(String),
    #[error("Empty expression")]
    EmptyExpression,
    #[error("Empty token")]
    EmptyToken,
    #[error("Invalid function name: '{0}")]
    InvalidFunctionName(String),
    #[error("Invalid token: '{0}")]
    InvalidToken(String),
    #[error("Expression ends with an operator: '{0}")]
    ExpressionEndsWithOperator(String),
}

#[derive(Error, Debug, Clone)]
pub enum ComputeError {
    #[error("Unsupported unary operator")]
    UnsupportedUnaryOperator,
    #[error("Invalid number of parameters for function '{0}': {1}")]
    InvalidNumberOfParametersForFunction(String, usize),
    #[error("Invalid number of parameters for operator {0}: {1}")]
    InvalidNumberOfParametersForOperator(char, usize),
    #[error("Invalid expression: {0}")]
    InvalidExpression(String),
}
