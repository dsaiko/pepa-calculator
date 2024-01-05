use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::Unit;

#[derive(Debug, Clone)]
pub struct Function {
    pub representation: String,
    pub fce: fn(params: Vec<f64>) -> f64,
    pub params_validation: fn(count: usize) -> bool,
    pub unit: Option<Unit>,
}

pub static FUNCTIONS: Lazy<HashMap<String, Function>> = Lazy::new(|| {
    let mut functions = HashMap::new();

    for function in [
        Function {
            representation: "sqrt".to_owned(),
            fce: |params| params[0].sqrt(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "sqr".to_owned(),
            fce: |params| params[0].powf(2.0),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "round".to_owned(),
            fce: |params| params[0].round(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "trunc".to_owned(),
            fce: |params| params[0].trunc(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "fract".to_owned(),
            fce: |params| params[0].fract(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "floor".to_owned(),
            fce: |params| params[0].floor(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "ceil".to_owned(),
            fce: |params| params[0].ceil(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "sin".to_owned(),
            fce: |params| params[0].sin(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "cos".to_owned(),
            fce: |params| params[0].cos(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "tan".to_owned(),
            fce: |params| params[0].tan(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "sinh".to_owned(),
            fce: |params| params[0].sinh(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "cosh".to_owned(),
            fce: |params| params[0].cosh(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "tanh".to_owned(),
            fce: |params| params[0].tanh(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "asin".to_owned(),
            fce: |params| params[0].asin(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "acos".to_owned(),
            fce: |params| params[0].acos(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "atan".to_owned(),
            fce: |params| params[0].atan(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "asinh".to_owned(),
            fce: |params| params[0].asinh(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "acosh".to_owned(),
            fce: |params| params[0].acosh(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "atanh".to_owned(),
            fce: |params| params[0].atanh(),
            params_validation: |count| count == 1,
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
            params_validation: |count| count > 0,
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
            params_validation: |count| count > 0,
            unit: None,
        },
        Function {
            representation: "ln".to_owned(),
            fce: |params| params[0].ln(),
            params_validation: |count| count == 1,
            unit: None,
        },
        Function {
            representation: "log".to_owned(),
            fce: |params| params[0].log(params[1]),
            params_validation: |count| count == 2,
            unit: None,
        },
        Function {
            representation: "pow".to_owned(),
            fce: |params| params[0].powf(params[1]),
            params_validation: |count| count == 2,
            unit: None,
        },
        Function {
            representation: "sum".to_owned(),
            fce: |params| params.iter().sum(),
            params_validation: |count| count > 0,
            unit: None,
        },
        Function {
            representation: "average".to_owned(),
            fce: |params| params.iter().sum::<f64>() / params.len() as f64,
            params_validation: |count| count > 0,
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
                    (params[index - 1] + params[index]) / 2.0
                }
            },
            params_validation: |count| count > 0,
            unit: None,
        },
        Function {
            representation: "count".to_owned(),
            fce: |params| params.len() as f64,
            params_validation: |count| count > 0,
            unit: None,
        },
    ] {
        functions.insert(function.representation.clone(), function);
    }

    functions
});

pub static FUNCTION_NAMES: Lazy<Vec<String>> = Lazy::new(|| {
    let mut names = Vec::new();
    for (k, _) in FUNCTIONS.iter() {
        names.push(k.clone());
    }

    // sorted reversed so sqrt is before sqr
    names.sort();
    names.reverse();

    names
});
