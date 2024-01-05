use std::collections::HashMap;

use crate::{ComputeError, ParserError};
use crate::compute::compute;
use crate::ComputeError::InvalidExpression;
use crate::expression::{Expression, NumericResult};
use crate::operators::CONVERSION_CHARACTER;
use crate::parser::parse;

#[derive(Debug, Clone)]
pub struct Statement {
    pub request: String,
    pub expression: Result<Expression, ParserError>,
    pub result: Result<NumericResult, ComputeError>,
}

pub struct Calc {
    statements: Vec<Statement>,
    variables: HashMap<String, NumericResult>,
}

impl Default for Calc {
    fn default() -> Self {
        Calc::new()
    }
}

impl Calc {
    pub fn new() -> Calc {
        Calc {
            statements: vec![],
            variables: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        *self = Calc::new();
    }

    pub fn compute(&mut self, statement: &str) -> Option<Statement> {
        let mut last_statement = None;

        for line in statement
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
        {
            let mut line = line.to_owned();
            for r in [" in ", " to ", " into "] {
                line = line.replace(r, CONVERSION_CHARACTER.to_string().as_str())
            }

            let compacted_line = line.split_whitespace().collect::<Vec<_>>().join("");

            let expression = parse(&compacted_line);
            let Ok(expression) = expression else {
                let statement = Statement {
                    request: line.to_owned(),
                    expression,
                    result: Err(InvalidExpression(line.to_owned())),
                };

                last_statement = Some(statement.clone());
                self.statements.push(statement);
                continue;
            };

            let result = compute(&expression);

            let statement = Statement {
                request: line.to_owned(),
                expression: Ok(expression),
                result,
            };
            last_statement = Some(statement.clone());
            self.statements.push(statement);
        }

        last_statement
    }
}
