use crate::{ComputeError, Unit};
use crate::expression::{Expression, ExpressionToken, NumericResult};

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
                let n = compute(ex)?;
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
                    let n = compute(ex)?;
                    operands.push(n);
                }

                invoke = true;
            }
        }

        if !invoke {
            continue;
        }

        // if function is set
        if let Some(f) = function {
            let (converted, unit) = convert_units(&operands, f.unit)?;

            let r = if (f.params_validation)(converted.len()) {
                (f.fce)(converted.iter().map(|n| n.value).collect())
            } else {
                return Err(ComputeError::InvalidNumberOfParametersForFunction(
                    f.representation.to_owned(),
                    converted.len(),
                ));
            };

            let n = NumericResult::new(r, unit);
            operands.clear();
            operands.push(n);
            result = Some(n);
            function = None;
            continue;
        }

        // if operation is set
        if let Some(o) = operator {
            let (converted, unit) = convert_units(&operands, None)?;
            let r = match converted.len() {
                1 => (o.unary_action)(converted[0].value),
                2 => (o.binary_action)(converted[0].value, converted[1].value),
                x => {
                    return Err(ComputeError::InvalidNumberOfParametersForOperator(
                        o.representation,
                        x,
                    ));
                }
            }?;

            let n = NumericResult::new(r, unit);
            operands.clear();
            operands.push(n);
            result = Some(n);
            operator = None;
            continue;
        }
    }

    // if function is at the end - invoke it with operands
    if let Some(f) = function {
        let (converted, unit) = convert_units(&operands, f.unit)?;
        let r = if (f.params_validation)(converted.len()) {
            (f.fce)(converted.iter().map(|n| n.value).collect())
        } else {
            return Err(ComputeError::InvalidNumberOfParametersForFunction(
                f.representation.to_owned(),
                converted.len(),
            ));
        };
        result = Some(NumericResult::new(r, unit));
    }

    result.ok_or(ComputeError::InvalidExpression(ex.to_string()))
}

fn convert_units(
    operands: &Vec<NumericResult>,
    force_unit: Option<Unit>,
) -> Result<(Vec<NumericResult>, Option<Unit>), ComputeError> {
    let mut unit = force_unit;
    if unit.is_none() {
        // if fce unit is not set, find unit from operands
        for n in operands.iter() {
            if n.unit.is_some() {
                unit = n.unit;
            }
        }
    }

    let Some(unit) = unit else {
        return Ok((operands.clone(), unit));
    };

    let mut converted = Vec::with_capacity(operands.len());
    for operand in operands {
        converted.push(operand.convert_to(unit)?)
    }

    Ok((converted, Some(unit)))
}
