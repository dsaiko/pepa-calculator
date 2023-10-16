use std::collections::HashMap;

use crate::compute::compute;
use crate::expression::Expression;
use crate::parser::parse;
use crate::units::Unit;

#[derive(Debug, Clone, PartialEq)]
pub enum ComputedResult {
    Numeric(f64, Option<Unit>),
    Variable(String),
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub request: String,
    pub expression: Option<Expression>,
    pub result: Option<ComputedResult>,
}

pub struct Computer {
    statements: Vec<Statement>,
    variables: HashMap<String, ComputedResult>,
}

impl Default for Computer {
    fn default() -> Self {
        Computer::new()
    }
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            statements: vec![],
            variables: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        *self = Computer::new();
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
