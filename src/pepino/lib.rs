use thiserror::Error;

pub use self::calc::Calc;
pub use self::units::Unit;
pub use self::units_temperature::TemperatureUnit;
pub use self::units_time::TimeUnit;

mod calc;
mod compute;
mod constants;
mod expression;
mod functions;
mod generators;
mod operators;
mod parser;
mod units;
mod units_temperature;
mod units_time;
mod utils;
#[cfg(test)]
mod utils_test;

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
    #[error("Unable to convert ${0} {1} to {2}")]
    UnitConversionError(f64, String, String),
}
