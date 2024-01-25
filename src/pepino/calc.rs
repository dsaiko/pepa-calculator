use std::collections::HashMap;

use crate::compute::compute;
use crate::expression::{Expression, NumericExpression};
use crate::operators::CONVERSION_CHARACTER;
use crate::parser::parse;
use crate::ComputeError::InvalidExpression;
use crate::{string, ComputeError, ParserError};

#[derive(Debug, Clone)]
pub struct Statement {
    pub request: String,
    pub expression: Result<Expression, ParserError>,
    pub result: Option<Result<NumericExpression, ComputeError>>,
}

pub struct Calc {
    statements: Vec<Statement>,
    variables: HashMap<String, NumericExpression>,
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

    fn prepare(&mut self, statement: &str) {
        for line in statement
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
        {
            let mut line = line.to_owned();
            for r in [" in ", " to ", " into "] {
                line = line.replace(r, &string!(CONVERSION_CHARACTER))
            }

            let compacted_line = line.split_whitespace().collect::<Vec<_>>().join("");

            let expression = parse(&compacted_line);
            let statement = Statement {
                request: line.to_owned(),
                expression,
                result: None,
            };
            self.statements.push(statement);
        }
    }

    pub fn compute(&mut self, statement: &str) -> Option<&Statement> {
        self.prepare(statement);

        for s in self.statements.iter_mut() {
            if s.result.is_none() {
                if let Ok(e) = &s.expression {
                    s.result = Some(compute(e));
                } else {
                    s.result = Some(Err(InvalidExpression(string!(s.request))));
                }
            }
        }

        self.statements.last()
    }

    pub fn prepare_statements(&mut self, statement: &str) -> Option<&Statement> {
        self.prepare(statement);
        self.statements.last()
    }
}
