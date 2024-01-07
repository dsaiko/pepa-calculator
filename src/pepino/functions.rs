use std::collections::HashMap;
use std::sync::OnceLock;

use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;

use crate::{Decimal, Unit};

#[derive(Debug, Clone)]
pub struct Function {
    pub representation: String,
    pub fce: fn(params: Vec<Decimal>) -> Decimal,
    pub params_validation: fn(params: Vec<Decimal>) -> bool,
    pub unit: Option<Unit>,
}

pub(super) fn functions() -> &'static HashMap<String, Function> {
    static MEM: OnceLock<HashMap<String, Function>> = OnceLock::new();
    MEM.get_or_init(|| {
        let mut functions = HashMap::new();

        for function in [
            Function {
                representation: "sqrt".to_owned(),
                fce: |params| params[0].sqrt().unwrap(),
                params_validation: |params| {
                    params.len() == 1 && !params.iter().any(|v| *v < dec!(0))
                },
                unit: None,
            },
            Function {
                representation: "sqr".to_owned(),
                fce: |params| params[0].powu(2),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "round".to_owned(),
                fce: |params| params[0].round(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "trunc".to_owned(),
                fce: |params| params[0].trunc(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "fract".to_owned(),
                fce: |params| params[0].fract(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "floor".to_owned(),
                fce: |params| params[0].floor(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "ceil".to_owned(),
                fce: |params| params[0].ceil(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "sin".to_owned(),
                fce: |params| params[0].sin(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "cos".to_owned(),
                fce: |params| params[0].cos(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "tan".to_owned(),
                fce: |params| params[0].tan(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "min".to_owned(),
                fce: |params| {
                    params
                        .into_iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                },
                params_validation: |params| params.len() > 0,
                unit: None,
            },
            Function {
                representation: "max".to_owned(),
                fce: |params| {
                    params
                        .into_iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                },
                params_validation: |params| params.len() > 0,
                unit: None,
            },
            Function {
                representation: "ln".to_owned(),
                fce: |params| params[0].ln(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "log".to_owned(),
                fce: |params| params[0].log10(),
                params_validation: |params| params.len() == 1,
                unit: None,
            },
            Function {
                representation: "pow".to_owned(),
                fce: |params| {
                    let x = params[0].powd(params[1]);
                    println!("{}", x);
                    x
                },
                params_validation: |params| params.len() == 2,
                unit: None,
            },
            Function {
                representation: "sum".to_owned(),
                fce: |params| params.iter().sum(),
                params_validation: |params| params.len() > 0,
                unit: None,
            },
            Function {
                representation: "average".to_owned(),
                fce: |params| params.iter().sum::<Decimal>() / Decimal::from(params.len()),
                params_validation: |params| params.len() > 0,
                unit: None,
            },
            Function {
                representation: "median".to_owned(),
                fce: |params| {
                    let mut params = params.clone();
                    params.sort_by(|a, b| a.partial_cmp(b).unwrap());

                    let index = params.len() / 2;

                    if params.len() % 2 == 1 {
                        params[index]
                    } else {
                        (params[index - 1] + params[index]) / dec!(2.0)
                    }
                },
                params_validation: |params| params.len() > 0,
                unit: None,
            },
            Function {
                representation: "count".to_owned(),
                fce: |params| params.len().into(),
                params_validation: |params| params.len() > 0,
                unit: None,
            },
        ] {
            functions.insert(function.representation.clone(), function);
        }

        functions
    })
}

pub(super) fn function_names() -> &'static Vec<String> {
    static MEM: OnceLock<Vec<String>> = OnceLock::new();
    MEM.get_or_init(|| {
        let mut names = Vec::new();
        for (k, _) in functions().iter() {
            names.push(k.clone());
        }

        // sorted reversed so sqrt is before sqr
        names.sort();
        names.reverse();

        names
    })
}
