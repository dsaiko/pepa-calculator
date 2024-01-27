use itertools::Itertools;
use rust_decimal::Decimal;

use crate::ComputeError;
use crate::expression::{Expression, ExpressionToken, NumericExpression};
use crate::functions::Function;
use crate::units::Unit;
use crate::utils::flatten_lines;

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
            let n = invoke_fce(f, variables.clone())?;
            variables.clear();
            variables.push(n.clone());
            result = Some(n.clone());
            function = None;
            continue;
        }

        // if operation is set
        if let Some(o) = operator {
            let (converted, _) = convert_variables(variables.clone())?;

            let n = match converted.len() {
                1 => invoke_unary(o.unary_action, &converted[0]),
                2 => invoke_binary(o.binary_action, &converted[0], &converted[1]),
                x => {
                    return Err(ComputeError::InvalidNumberOfParametersForOperator(
                        o.representation,
                        x,
                    ));
                }
            }?;

            variables.clear();
            variables.push(n.clone());
            result = Some(n.clone());
            operator = None;
            continue;
        }

        // if conversion chain is set
        if let Some(chain) = conversion_chain {
            if variables.len() == 1 {
                let chain = flatten_lines(chain);
                let mut values = Vec::new();

                for chain_variant in chain.clone() {
                    let mut v = variables[0].clone();

                    let mut ok = true;
                    for unit in chain_variant.clone() {
                        let Ok(v_converted) = v.convert_to(&unit, true) else {
                            ok = false;
                            break;
                        };
                        v = v_converted;
                    }

                    if ok {
                        values.extend(v.values());
                    }
                }

                if values.is_empty() {
                    return Err(ComputeError::OperatorsConversionError(
                        variables,
                        chain
                            .iter()
                            .map(|x| x.iter().map(|x| Some(*x)).collect())
                            .collect(),
                    ));
                }

                variables = vec![NumericExpression::with_multiple_units(values)];

                conversion_chain = None;
                continue;
            }

            // convert multiple variables in chain

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
                return Err(ComputeError::OperatorsConversionError(
                    variables,
                    chain
                        .iter()
                        .map(|x| x.iter().map(|x| Some(*x)).collect())
                        .collect(),
                ));
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
        let r = invoke_fce(f, variables)?;
        result = Some(r);
    }

    result.ok_or(ComputeError::InvalidExpression(ex.to_string()))
}

fn convert_variables(
    variables: Vec<NumericExpression>,
) -> Result<(Vec<NumericExpression>, Vec<Unit>), ComputeError> {
    let mut to: Vec<Unit> = Vec::new();

    for n in variables.iter() {
        let units: Vec<_> = n.units().iter().flatten().copied().collect();
        if !units.is_empty() {
            to = units
        }
    }

    if !to.is_empty() {
        // reduce to valid only units
        let mut valid = Vec::new();

        for u in to.iter() {
            let mut ok = true;

            for variable in variables.iter() {
                if variable.convert_to(u, false).is_err() {
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
                vec![to.iter().map(|x| Some(*x)).collect()],
            ));
        }

        to = valid;
    }

    match to.len() {
        0 => Ok((variables.clone(), to)),
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
                return Ok((converted, to));
            }

            Err(ComputeError::OperatorsConversionError(
                variables.clone(),
                vec![vec![Some(u)]],
            ))
        }
        _ => {
            let mut converted = Vec::new();

            for variable in variables.iter() {
                let mut res = Vec::new();
                for u in to.iter() {
                    if let Ok(v) = variable.convert_to(u, false) {
                        res.extend(v.values());
                    };
                }

                res = res.iter().unique().copied().collect();

                match res.len() {
                    0 => {
                        return Err(ComputeError::OperatorsConversionError(
                            variables.clone(),
                            vec![to.iter().map(|x| Some(*x)).collect()],
                        ));
                    }
                    1 => {
                        converted.push(NumericExpression::with_unit(res[0].0, res[0].1));
                    }
                    _ => {
                        converted.push(NumericExpression::with_multiple_units(res));
                    }
                }
            }

            Ok((converted, to))
        }
    }
}

fn invoke_unary(
    f: fn(Decimal) -> Result<Decimal, ComputeError>,
    p: &NumericExpression,
) -> Result<NumericExpression, ComputeError> {
    let mut res = Vec::new();
    for (n, u) in p.values() {
        let r = f(n)?;
        res.push((r, u))
    }

    res = res.iter().unique().copied().collect();
    Ok(NumericExpression::with_multiple_units(res))
}

fn invoke_binary(
    f: fn(Decimal, Decimal) -> Result<Decimal, ComputeError>,
    p1: &NumericExpression,
    p2: &NumericExpression,
) -> Result<NumericExpression, ComputeError> {
    let mut res = Vec::new();

    for (n1, u1) in p1.values() {
        for (n2, u2) in p2.values() {
            match u1 {
                None => match u2 {
                    None => res.push((f(n1, n2)?, None)),
                    Some(u2) => res.push((f(n1, n2)?, Some(u2))),
                },
                Some(u1) => match u2 {
                    None => res.push((f(n1, n2)?, Some(u1))),
                    Some(u2) => {
                        let Some(n1) = u1.conversion(&n1, &u2) else {
                            continue;
                        };
                        res.push((f(n1, n2)?, Some(u2)));
                    }
                },
            }
        }
    }

    if res.is_empty() {
        return Err(ComputeError::OperatorsConversionError(
            vec![p1.clone(), p2.clone()],
            vec![p1.units(), p2.units()],
        ));
    }

    res = res.iter().unique().copied().collect();
    Ok(NumericExpression::with_multiple_units(res))
}

fn invoke_fce(
    f: &Function,
    variables: Vec<NumericExpression>,
) -> Result<NumericExpression, ComputeError> {
    let (converted, _) = convert_variables(variables.clone())?;
    let converted = converted.iter().map(|x| x.values()).collect::<Vec<_>>();

    // create combination of arguments
    let lines = flatten_lines(&converted)
        .iter()
        .map(|v| {
            v.iter()
                .map(|(n, u)| NumericExpression::with_unit(*n, *u))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = Vec::new();

    for line in lines {
        // validate line
        if let Ok((converted_line, units)) = convert_variables(line.clone()) {
            let params = converted_line.iter().map(|n| n.values()[0].0).collect();
            let n = if (f.params_validation)(&params) {
                (f.fce)(params)
            } else {
                return Err(ComputeError::InvalidParametersForFunction(
                    f.representation.to_owned(),
                    format!("{:?}", converted),
                ));
            };

            res.push((n, units.first().cloned()))
        }
    }

    res = res.iter().unique().copied().collect();

    if res.is_empty() {
        return Err(ComputeError::InvalidParametersForFunction(
            f.representation.to_owned(),
            format!("{:?}", converted),
        ));
    }

    Ok(NumericExpression::with_multiple_units(res))
}
