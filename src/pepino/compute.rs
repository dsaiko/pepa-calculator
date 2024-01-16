use crate::expression::{Expression, ExpressionToken, NumericExpression};
use crate::utils::flatten_lines;
use crate::{ComputeError, Unit};

pub(super) fn compute(ex: &Expression) -> Result<NumericExpression, ComputeError> {
    let mut variables = Vec::new();
    let mut result: Option<NumericExpression> = None;
    let mut operator = None;
    let mut function = None;
    let mut conversion_chain = None;

    for t in ex.tokens.iter() {
        let mut invoke = false;

        match t {
            ExpressionToken::Operator(o) => operator = Some(o),
            ExpressionToken::Function(f) => function = Some(f),
            ExpressionToken::Numeric(n) => {
                variables.push(n.clone());
                if result.is_none() {
                    // initial result = first operand
                    result = Some(n.clone());
                }

                invoke = true;
            }
            ExpressionToken::Expression(ex) => {
                let n = compute(ex)?;
                variables.push(n.clone());
                if result.is_none() {
                    // initial result = first operand
                    result = Some(n.clone());
                }

                invoke = true;
            }
            ExpressionToken::Generator(g) => {
                let n = NumericExpression::with_unit((g.fce)(), None); // Unit: None
                variables.push(n.clone());
                if result.is_none() {
                    // initial result = first operand
                    result = Some(n.clone());
                }

                invoke = true;
            }
            ExpressionToken::List(list) => {
                for ex in list {
                    let n = compute(ex)?;
                    variables.push(n);
                }

                invoke = true;
            }
            ExpressionToken::ConversionChain(c) => {
                conversion_chain = Some(c);
                invoke = true;
            }
        }

        if !invoke {
            continue;
        }

        // if function is set
        if let Some(f) = function {
            let (converted, unit) = convert_operands(variables.clone(), &f.unit)?;

            let params = converted.iter().map(|n| n.value()).collect();
            let r = if (f.params_validation)(params) {
                (f.fce)(converted.iter().map(|n| n.value()).collect())
            } else {
                return Err(ComputeError::InvalidParametersForFunction(
                    f.representation.to_owned(),
                    format!("{:?}", converted),
                ));
            };

            let n = NumericExpression::with_unit(r, unit);
            variables.clear();
            variables.push(n.clone());
            result = Some(n.clone());
            function = None;
            continue;
        }

        // if operation is set
        if let Some(o) = operator {
            let (converted, unit) = convert_operands(variables.clone(), &vec![])?;
            let r = match converted.len() {
                1 => (o.unary_action)(converted[0].value()),
                2 => (o.binary_action)(converted[0].value(), converted[1].value()),
                x => {
                    return Err(ComputeError::InvalidNumberOfParametersForOperator(
                        o.representation,
                        x,
                    ));
                }
            }?;

            let n = NumericExpression::with_unit(r, unit);
            variables.clear();
            variables.push(n.clone());
            result = Some(n.clone());
            operator = None;
            continue;
        }

        // if conversion chain is set
        if let Some(chain) = conversion_chain {
            let chain = flatten_lines(chain);
            let mut all_ok = false;

            for chain_variant in chain.clone() {
                let mut converted = Vec::new();
                for variable in variables.clone() {
                    let mut v = variable;

                    let mut ok = true;
                    for unit in chain_variant.clone() {
                        let Ok(v_converted) = v.convert_to(&unit, true) else {
                            ok = false;
                            break;
                        };
                        v = v_converted;
                    }

                    if ok {
                        converted.push(v.clone());
                    } else {
                        break;
                    }
                }

                if converted.len() == variables.len() {
                    // all converted OK
                    variables = converted;
                    all_ok = true;
                    break;
                }
            }

            if !all_ok {
                return Err(ComputeError::OperatorsConversionError(variables, chain));
            }

            conversion_chain = None;
            continue;
        }
    }

    if variables.len() == 1 {
        result = Some(variables[0].clone())
    }

    // if function is at the end - invoke it with operands
    if let Some(f) = function {
        let (converted, unit) = convert_operands(variables.clone(), &f.unit)?;
        let params = converted.iter().map(|n| n.value()).collect();
        let r = if (f.params_validation)(params) {
            (f.fce)(converted.iter().map(|n| n.value()).collect())
        } else {
            return Err(ComputeError::InvalidParametersForFunction(
                f.representation.to_owned(),
                format!("{:?}", converted),
            ));
        };

        result = Some(NumericExpression::with_unit(r, unit));
    }

    result.ok_or(ComputeError::InvalidExpression(ex.to_string()))
}

// TODO: do not return unit
fn convert_operands(
    variables: Vec<NumericExpression>,
    to: &[Unit],
) -> Result<(Vec<NumericExpression>, Option<Unit>), ComputeError> {
    let mut to = to.to_vec();

    if to.is_empty() {
        // if fce unit is not set, find last unit from operands
        for n in variables.iter() {
            match n {
                NumericExpression::Decimal(_) => {}
                NumericExpression::DecimalWithUnit(_, u) => to = vec![*u],
                NumericExpression::DecimalWithMultipleUnits(u) => {
                    to = u.iter().map(|x| x.1).collect()
                }
            }
        }
    }

    if !to.is_empty() {
        // reduce to valid only units
        let mut valid = Vec::new();

        for u in to.iter() {
            let mut ok = true;

            for variable in variables.iter() {
                if let Err(_) = variable.convert_to(u, false) {
                    ok = false;
                    break;
                };
            }

            if ok {
                valid.push(*u);
            }
        }

        if valid.is_empty() {
            return Err(ComputeError::OperatorsConversionError(
                variables.clone(),
                vec![to],
            ));
        }

        to = valid;
    }

    match to.len() {
        0 => Ok((variables.clone(), None)),
        1 => {
            let mut ok = true;
            let mut converted = Vec::with_capacity(variables.len());
            let u = to[0];

            for variable in variables.iter() {
                let Ok(v) = variable.convert_to(&u, false) else {
                    ok = false;
                    break;
                };
                converted.push(v);
            }

            if ok {
                return Ok((converted, Some(u)));
            }

            Err(ComputeError::OperatorsConversionError(
                variables.clone(),
                vec![vec![u]],
            ))
        }
        _ => {
            // compute for all in "to"
            // create result with multiple values
            todo!()
        }
    }
}
