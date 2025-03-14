use crate::{Decimal, ParserError};
use crate::constants::constants;
use crate::expression::{Expression, ExpressionToken, NumericExpression};
use crate::functions::{function_names, functions};
use crate::generators::generators;
use crate::operators::{CONVERSION_CHARACTER, operators, Priority};
use crate::units::Unit;
use crate::utils::split_string_by_comma;

pub(super) fn parse(ex: &str) -> Result<Expression, ParserError> {
    let mut expression = Expression::new();

    let mut ex = ex.to_owned();

    // replace generator fce by name only
    for (name, g) in generators().iter() {
        ex = ex.replace(&g.fce_name, name);
    }

    if ex.is_empty() {
        return Err(ParserError::EmptyExpression);
    }

    let mut ex = ex.to_owned();
    // simplify +- -+ ++ --
    loop {
        let fix = ex
            .replace("--", "+")
            .replace("++", "+")
            .replace("-+", "-")
            .replace("+-", "-");

        if ex != fix {
            ex = fix
        } else {
            break;
        }
    }

    // read while not operator
    let mut token = String::new();

    // parse next character
    let mut chars = ex.chars();
    while let Some(c) = chars.next() {
        // process operators
        if let Some(o) = operators().get(&c) {
            if !token.is_empty() {
                let ex = parse_token(&token)?;
                expression.push(ex);
                token.clear()
            }
            expression.push(ExpressionToken::Operator((*o).clone()));
            continue;
        }

        // process parentheses
        if c == ')' {
            // can not start with close
            return Err(ParserError::UnbalancedParentheses(ex.to_owned()));
        }

        if c == '(' {
            // prev token:
            if !token.is_empty() {
                let ex = parse_token(&token)?;
                expression.push(ex);
                token.clear()
            }

            // read to next matching parentheses
            let mut count = 1;
            let mut ex = String::new();

            for c in chars.by_ref() {
                match c {
                    '(' => {
                        ex.push(c);
                        count += 1
                    }
                    ')' => {
                        count -= 1;
                        if count == 0 {
                            // we have the full inner group
                            if !ex.is_empty() {
                                // list of parameters
                                let comma_list = split_string_by_comma(&ex);
                                if comma_list.len() > 1 {
                                    let mut list = Vec::new();
                                    // list of arguments
                                    for ex in comma_list {
                                        let ex = parse(&ex)?;
                                        list.push(ex);
                                    }
                                    expression.push(ExpressionToken::List(list));
                                } else {
                                    let ex = parse(&ex)?;
                                    expression.push(ExpressionToken::Expression(ex));
                                }
                            }
                            break;
                        } else {
                            ex.push(c);
                        }
                    }
                    _ => ex.push(c),
                }
            }

            if count != 0 {
                return Err(ParserError::UnbalancedParentheses(ex.to_owned()));
            }
            continue;
        }

        // just append token
        token.push(c);
    }

    if !token.is_empty() {
        let ex = parse_token(&token)?;
        expression.push(ex);
    }

    // normalize: if there are two operators one after each other, threat the second one as unary expression
    // example: 5*-1 => 5 * (-1)
    let mut normalized = Expression::new();
    let mut tokens = expression.tokens.into_iter();
    while let Some(e) = tokens.next() {
        match &e {
            ExpressionToken::Operator(_) => {
                let Some(e2) = tokens.next() else {
                    normalized.push(e);
                    continue;
                };

                match e2 {
                    ExpressionToken::Operator(_) => {
                        // if e is operator and e2 is also operator
                        normalized.push(e);
                        let Some(e3) = tokens.next() else {
                            normalized.push(e2);
                            continue;
                        };

                        // create a subexpression
                        normalized.push(ExpressionToken::Expression(Expression::from_tokens(vec![
                            e2, e3,
                        ])))
                    }
                    _ => {
                        // nothing
                        normalized.push(e);
                        normalized.push(e2);
                    }
                }
            }
            ExpressionToken::Numeric(_) => normalized.push(e),
            ExpressionToken::Expression(e2) => {
                if e2.tokens.len() == 1 {
                    // unwrap
                    normalized.push(e2.tokens.first().unwrap().clone())
                } else {
                    // no change
                    normalized.push(e);
                }
            }
            ExpressionToken::Function(_) => normalized.push(e),
            ExpressionToken::Generator(_) => normalized.push(e),
            ExpressionToken::List(_) => normalized.push(e),
            ExpressionToken::ConversionChain(_) => normalized.push(e),
        }
    }

    expression = normalized;

    // TODO: simplify this mess

    // prioritize functions
    {
        let mut prioritized = Expression::new();
        let mut buff2 = Vec::new();
        let mut tokens = expression.tokens.into_iter();

        while let Some(e) = tokens.next() {
            buff2.push(e.clone());

            if buff2.len() == 2 {
                let mut priority_group = false;
                // check if first element is a function
                if let ExpressionToken::Function(_) = buff2[0] {
                    priority_group = true
                }

                if priority_group {
                    if let ExpressionToken::Operator(_) = buff2[1] {
                        // another expression follows
                        // floor - 1.9
                        let Some(next) = tokens.next() else {
                            return Err(ParserError::ExpressionEndsWithOperator(ex));
                        };

                        buff2 = vec![ExpressionToken::Expression(Expression::from_tokens(vec![
                            buff2[0].clone(),
                            ExpressionToken::Expression(Expression::from_tokens(vec![
                                buff2[1].clone(),
                                next,
                            ])),
                        ]))];
                    } else {
                        buff2 = vec![ExpressionToken::Expression(Expression::from_tokens(buff2))];
                    }
                } else {
                    prioritized.push(buff2[0].clone());
                    // shift buff2 - remove head
                    buff2 = vec![buff2[1].clone()];
                }
            }
        }

        // append rest from buff2
        for t in buff2 {
            prioritized.push(t);
        }

        expression = prioritized;
    }

    // prioritize conversion chains
    {
        let mut prioritized = Expression::new();
        let mut buff2 = Vec::new();

        for e in expression.tokens.into_iter() {
            buff2.push(e.clone());

            if buff2.len() == 2 {
                // check if second element is a conversion chain
                if matches!(buff2[1], ExpressionToken::ConversionChain(_))
                    && !matches!(buff2[0], ExpressionToken::Operator(_))
                {
                    buff2 = vec![ExpressionToken::Expression(Expression::from_tokens(buff2))];
                } else if matches!(buff2[0], ExpressionToken::ConversionChain(_))
                    && !matches!(buff2[1], ExpressionToken::Operator(_))
                {
                    // celsius(55)
                    buff2.reverse();
                    buff2 = vec![ExpressionToken::Expression(Expression::from_tokens(buff2))];
                } else {
                    prioritized.push(buff2[0].clone());
                    // shift buff2 - remove head
                    buff2 = vec![buff2[1].clone()];
                }
            }
        }

        // append rest from buff2
        for t in buff2 {
            prioritized.push(t);
        }

        expression = prioritized;
    }

    // prioritize operands
    for priority in [Priority::Highest, Priority::High] {
        let mut prioritized = Expression::new();
        let mut buff3 = Vec::new();
        let tokens = expression.tokens.into_iter();

        for e in tokens {
            buff3.push(e);
            if buff3.len() == 3 {
                let mut priority_group = false;
                // check if middle element is a priority operator
                if let ExpressionToken::Operator(e) = buff3[1].clone() {
                    if e.priority == priority {
                        priority_group = true
                    }
                }

                if priority_group {
                    buff3 = vec![ExpressionToken::Expression(Expression::from_tokens(buff3))];
                } else {
                    prioritized.push(buff3[0].clone());
                    // shift buff3 - remove head
                    buff3 = vec![buff3[1].clone(), buff3[2].clone()];
                }
            }
        }
        // append rest from buff3
        for t in buff3 {
            prioritized.push(t);
        }

        expression = prioritized;
    }

    if expression.tokens.len() == 1 {
        if let ExpressionToken::Expression(e) = expression.tokens.first().unwrap() {
            return Ok(e.clone());
        }
    }

    Ok(expression)
}

fn parse_token(token: &str) -> Result<ExpressionToken, ParserError> {
    if token.is_empty() {
        return Err(ParserError::EmptyToken);
    }

    // numeric expression
    if let Ok(n) = token.parse::<Decimal>() {
        return Ok(ExpressionToken::Numeric(NumericExpression::with_unit(
            n, None,
        )));
    }

    // function
    if let Some(f) = functions().get(token) {
        return Ok(ExpressionToken::Function((*f).clone()));
    }

    // constant
    if let Some(n) = constants().get(token) {
        return Ok(ExpressionToken::Numeric(NumericExpression::with_unit(
            *n, None,
        )));
    }

    // generator
    if let Some(g) = generators().get(token) {
        return Ok(ExpressionToken::Generator((*g).clone()));
    }

    // check if token does not start with a function name
    // sin cos ( PI )
    for fce_name in function_names().iter() {
        if token.starts_with(fce_name) {
            let Some(fce) = functions().get(fce_name) else {
                return Err(ParserError::InvalidFunctionName((*fce_name).to_owned()));
            };

            let Ok(ex) = parse_token(token.strip_prefix(fce_name).unwrap()) else {
                return Err(ParserError::InvalidToken(token.to_owned()));
            };

            return Ok(ExpressionToken::Expression(Expression::from_tokens(vec![
                ExpressionToken::Function((*fce).clone()),
                ex,
            ])));
        }
    }

    // check for a number group
    // 55celsius or celsius55
    let mut number = String::new();
    let mut unit = String::new();
    let mut capturing_number = token.chars().next().unwrap().is_numeric();
    if capturing_number {
        for c in token.chars() {
            if capturing_number && c.is_numeric() {
                number.push(c);
            } else {
                capturing_number = false;
                unit.push(c);
            }
        }
    } else {
        for c in token.chars() {
            if !capturing_number && !c.is_numeric() {
                unit.push(c);
            } else {
                capturing_number = true;
                number.push(c);
            }
        }
    }

    let units = unit
        .split(CONVERSION_CHARACTER)
        .filter(|&x| !x.is_empty())
        .map(Unit::from_string)
        .collect::<Vec<_>>();

    if units.is_empty() || units.iter().any(|u| u.is_empty()) {
        return Err(ParserError::InvalidToken(token.to_owned()));
    }

    // number must be a numeric value
    if let Ok(n) = number.parse::<Decimal>() {
        if units.len() == 1 {
            return Ok(ExpressionToken::Numeric(NumericExpression::with_units(
                n,
                units[0].clone(),
            )));
        }

        return Ok(ExpressionToken::Expression(Expression::from_tokens(vec![
            ExpressionToken::Numeric(NumericExpression::with_unit(n, None)),
            ExpressionToken::ConversionChain(units),
        ])));
    };

    Ok(ExpressionToken::ConversionChain(units))
}
