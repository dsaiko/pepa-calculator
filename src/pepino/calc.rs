use std::collections::HashMap;

use crate::compute::compute;
use crate::expression::{Expression, NumericResult};
use crate::parser::parse;

#[derive(Debug, Clone)]
pub struct Statement {
    pub request: String,
    pub expression: Option<Expression>,
    pub result: Option<NumericResult>,
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

    pub fn last_statement(&self) -> Option<&Statement> {
        self.statements.last()
    }

    pub fn compute(&mut self, statement: &str) {
        for line in statement
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
        {
            let compacted_line = line.split_whitespace().collect::<Vec<_>>().join("");

            let expression = parse(compacted_line.as_str());

            let result = if let Ok(expression) = &expression {
                compute(expression).ok()
            } else {
                None
            };

            self.statements.push(Statement {
                request: line.to_owned(),
                expression: expression.ok(),
                result,
            })
        }
    }
}
