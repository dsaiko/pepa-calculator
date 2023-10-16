use crate::{ComputedResult, ParsingError};
use crate::expression::{Expression, ExpressionToken};

pub(super) fn compute(ex: &Expression) -> anyhow::Result<ComputedResult> {
    let mut operands = Vec::new();
    let mut result = None;
    let mut operation = None;

    for t in ex.tokens.iter() {
        let mut invoke = false;

        match t {
            ExpressionToken::Operator(o) => operation = Some(o),
            ExpressionToken::ComputedResult(r) => {
                operands.push(r.clone());
                if result.is_none() {
                    // initial result = first operand
                    result = Some(r.clone());
                }

                invoke = true;
            }
            ExpressionToken::Expression(ex) => {
                let Ok(r) = compute(ex) else {
                    return Err(ParsingError::InvalidExpression.into());
                };

                operands.push(r.clone());
                if result.is_none() {
                    // initial result = first operand
                    result = Some(r.clone());
                }

                invoke = true;
            }
        }

        if !invoke {
            continue;
        }

        // if operation is set
        if let Some(operator) = operation {
            let mut values = Vec::new();
            for o in &operands {
                match o {
                    ComputedResult::Numeric(v, _) => values.push(*v),
                    ComputedResult::Variable(_) => todo!(),
                }
            }

            let r = match values.len() {
                1 => (operator.unary_action)(values[0]),
                2 => (operator.binary_action)(values[0], values[1]),
                _ => return Err(ParsingError::InvalidExpression.into()),
            };

            let Ok(r) = r else {
                // operator has returned None - not supported operation
                return Err(ParsingError::InvalidExpression.into());
            };

            let r = ComputedResult::Numeric(r, None); // TODO: Unit
            operands.clear();
            operands.push(r.clone());
            result = Some(r);
        }
    }

    result.ok_or(ParsingError::InvalidExpression.into())
}
