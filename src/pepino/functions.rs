use std::collections::HashMap;
use std::sync::OnceLock;

use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;

use crate::units::UnitDefinition;
use crate::{string, Decimal, Unit};

#[derive(Debug, Clone)]
pub struct Function {
    pub representation: String,
    pub fce: fn(params: Vec<Decimal>) -> Decimal,
    pub params_validation: fn(params: Vec<Decimal>) -> bool,
    pub unit: UnitDefinition,
}

pub(super) fn functions() -> &'static HashMap<String, Function> {
    static MEM: OnceLock<HashMap<String, Function>> = OnceLock::new();
    MEM.get_or_init(|| {
        let mut functions = HashMap::new();

        for function in [
            Function {
                representation: string!("sqrt"),
                fce: |params| params[0].sqrt().unwrap(),
                params_validation: |params| {
                    params.len() == 1 && !params.iter().any(|v| *v < dec!(0))
                },
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("sqr"),
                fce: |params| params[0].powu(2),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("round"),
                fce: |params| params[0].round(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("trunc"),
                fce: |params| params[0].trunc(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("fract"),
                fce: |params| params[0].fract(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("floor"),
                fce: |params| params[0].floor(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("ceil"),
                fce: |params| params[0].ceil(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("sin"),
                fce: |params| params[0].sin(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("cos"),
                fce: |params| params[0].cos(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("tan"),
                fce: |params| params[0].tan(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("min"),
                fce: |params| {
                    params
                        .into_iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                },
                params_validation: |params| !params.is_empty(),
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("max"),
                fce: |params| {
                    params
                        .into_iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                },
                params_validation: |params| !params.is_empty(),
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("ln"),
                fce: |params| params[0].ln(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("log"),
                fce: |params| params[0].log10(),
                params_validation: |params| params.len() == 1,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("pow"),
                fce: |params| params[0].powd(params[1]),
                params_validation: |params| params.len() == 2,
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("sum"),
                fce: |params| params.iter().sum(),
                params_validation: |params| !params.is_empty(),
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("average"),
                fce: |params| params.iter().sum::<Decimal>() / Decimal::from(params.len()),
                params_validation: |params| !params.is_empty(),
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("median"),
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
                params_validation: |params| !params.is_empty(),
                unit: UnitDefinition::None,
            },
            Function {
                representation: string!("count"),
                fce: |params| params.len().into(),
                params_validation: |params| !params.is_empty(),
                unit: UnitDefinition::None,
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
