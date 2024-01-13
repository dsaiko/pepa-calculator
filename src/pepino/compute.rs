use crate::expression::{Expression, ExpressionToken, NumericExpression};
use crate::units::UnitDefinition;
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
                variables.push(*n);
                if result.is_none() {
                    // initial result = first operand
                    result = Some(*n);
                }

                invoke = true;
            }
            ExpressionToken::Expression(ex) => {
                let n = compute(ex)?;
                variables.push(n);
                if result.is_none() {
                    // initial result = first operand
                    result = Some(n);
                }

                invoke = true;
            }
            ExpressionToken::Generator(g) => {
                let n = NumericExpression::new((g.fce)(), None); // Unit: None
                variables.push(n);
                if result.is_none() {
                    // initial result = first operand
                    result = Some(n);
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

            let n = NumericExpression::new(r, unit);
            variables.clear();
            variables.push(n);
            result = Some(n);
            function = None;
            continue;
        }

        // if operation is set
        if let Some(o) = operator {
            let (converted, unit) = convert_operands(variables.clone(), &UnitDefinition::None)?;
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

            let n = NumericExpression::new(r, unit);
            variables.clear();
            variables.push(n);
            result = Some(n);
            operator = None;
            continue;
        }

        // if conversion chain is set
        if let Some(chain) = conversion_chain {
            let chain = flatten_lines(
                &chain
                    .iter()
                    .map(|u| match u {
                        UnitDefinition::None => vec![],
                        UnitDefinition::Single(u) => vec![*u],
                        UnitDefinition::Multi(u) => u.clone(),
                    })
                    .collect(),
            );
            let mut all_ok = false;

            for chain_variant in chain.clone() {
                let mut converted = Vec::new();
                for variable in variables.clone() {
                    let mut v = variable;

                    let mut ok = true;
                    for unit in chain_variant.clone() {
                        let Ok(v_converted) = v.convert_to(unit, true) else {
                            ok = false;
                            break;
                        };
                        v = v_converted;
                    }

                    if ok {
                        converted.push(v);
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
        result = Some(variables[0])
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

        result = Some(NumericExpression::new(r, unit));
    }

    result.ok_or(ComputeError::InvalidExpression(ex.to_string()))
}

fn convert_operands(
    operands: Vec<NumericExpression>,
    unit: &UnitDefinition,
) -> Result<(Vec<NumericExpression>, Option<Unit>), ComputeError> {
    let mut unit = unit.clone();

    if unit.is_none() {
        // if fce unit is not set, find last unit from operands
        for n in operands.iter() {
            match n {
                NumericExpression::Decimal(_) => {}
                NumericExpression::DecimalWithUnit(_, u) => {
                    unit = UnitDefinition::Single(*u);
                }
            }
        }
    }

    match unit {
        UnitDefinition::None => Ok((operands.clone(), None)),
        UnitDefinition::Single(u) => {
            let mut ok = true;
            let mut converted = Vec::with_capacity(operands.len());

            for operand in operands.iter() {
                let Ok(v) = operand.convert_to(u, false) else {
                    ok = false;
                    break;
                };
                converted.push(v);
            }

            if ok {
                return Ok((converted, Some(u)));
            }

            Err(ComputeError::OperatorsConversionError(
                operands.clone(),
                vec![vec![u]],
            ))
        }
        UnitDefinition::Multi(_) => todo!(),
    }
}
