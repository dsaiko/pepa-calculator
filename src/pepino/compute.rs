use crate::expression::{Expression, ExpressionToken, NumericResult};
use crate::ComputeError;

pub(super) fn compute(ex: &Expression) -> Result<NumericResult, ComputeError> {
    let mut operands = Vec::new();
    let mut result: Option<NumericResult> = None;
    let mut operator = None;
    let mut function = None;

    for t in ex.tokens.iter() {
        let mut invoke = false;

        match t {
            ExpressionToken::Operator(o) => operator = Some(o),
            ExpressionToken::Function(f) => function = Some(f),
            ExpressionToken::Numeric(n) => {
                operands.push(*n);
                if result.is_none() {
                    // initial result = first operand
                    result = Some(*n);
                }

                invoke = true;
            }
            ExpressionToken::Expression(ex) => {
                let ex = compute(ex);
                let Ok(n) = ex else {
                    return Err(ex.err().unwrap());
                };

                operands.push(n);
                if result.is_none() {
                    // initial result = first operand
                    result = Some(n);
                }

                invoke = true;
            }
            ExpressionToken::Generator(g) => {
                let n = NumericResult::new((g.fce)(), None); // Unit: None
                operands.push(n);
                if result.is_none() {
                    // initial result = first operand
                    result = Some(n);
                }

                invoke = true;
            }
            ExpressionToken::List(list) => {
                for ex in list {
                    let ex = compute(ex);
                    let Ok(n) = ex else {
                        return Err(ex.err().unwrap());
                    };
                    operands.push(n);
                }

                invoke = true;
            }
        }

        if !invoke {
            continue;
        }

        // if operation is set
        if let Some(f) = function {
            // TODO: units
            let r = if (f.params_validation)(operands.len()) {
                (f.fce)(operands.iter().map(|n| n.value).collect())
            } else {
                return Err(ComputeError::InvalidNumberOfParametersForFunction(
                    f.representation.to_owned(),
                    operands.len(),
                ));
            };

            let n = NumericResult::new(r, None); // TODO: Unit
            operands.clear();
            operands.push(n);
            result = Some(n);
            function = None;
            continue;
        }

        // if operation is set
        if let Some(o) = operator {
            // TODO: units
            let r = match operands.len() {
                1 => (o.unary_action)(operands[0].value),
                2 => (o.binary_action)(operands[0].value, operands[1].value),
                x => {
                    return Err(ComputeError::InvalidNumberOfParametersForOperator(
                        o.representation,
                        x,
                    ));
                }
            };

            let Ok(r) = r else {
                // operator has returned None - not supported operation
                return Err(r.err().unwrap());
            };

            let n = NumericResult::new(r, None); // TODO: Unit
            operands.clear();
            operands.push(n);
            result = Some(n);
            operator = None;
            continue;
        }
    }

    result.ok_or(ComputeError::InvalidExpression(ex.to_string()))
}
