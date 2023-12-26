use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Function {
    pub representation: &'static str,
    pub fce: fn(params: Vec<f64>) -> f64,
    pub params_validation: fn(count: usize) -> bool,
}

pub static FUNCTIONS: Lazy<HashMap<&str, Function>> = Lazy::new(|| {
    let mut functions = HashMap::new();

    for function in [
        Function {
            representation: "sqrt",
            fce: |params| params[0].sqrt(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "sqr",
            fce: |params| params[0].powf(2.0),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "round",
            fce: |params| params[0].round(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "trunc",
            fce: |params| params[0].trunc(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "fract",
            fce: |params| params[0].fract(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "floor",
            fce: |params| params[0].floor(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "ceil",
            fce: |params| params[0].ceil(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "sin",
            fce: |params| params[0].sin(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "cos",
            fce: |params| params[0].cos(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "tan",
            fce: |params| params[0].tan(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "sinh",
            fce: |params| params[0].sinh(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "cosh",
            fce: |params| params[0].cosh(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "tanh",
            fce: |params| params[0].tanh(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "asin",
            fce: |params| params[0].asin(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "acos",
            fce: |params| params[0].acos(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "atan",
            fce: |params| params[0].atan(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "asinh",
            fce: |params| params[0].asinh(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "acosh",
            fce: |params| params[0].acosh(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "atanh",
            fce: |params| params[0].atanh(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "min",
            fce: |params| {
                params
                    .into_iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            },
            params_validation: |count| count > 0,
        },
        Function {
            representation: "max",
            fce: |params| {
                params
                    .into_iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            },
            params_validation: |count| count > 0,
        },
        Function {
            representation: "ln",
            fce: |params| params[0].ln(),
            params_validation: |count| count == 1,
        },
        Function {
            representation: "log",
            fce: |params| params[0].log(params[1]),
            params_validation: |count| count == 2,
        },
        Function {
            representation: "pow",
            fce: |params| params[0].powf(params[1]),
            params_validation: |count| count == 2,
        },
        Function {
            representation: "sum",
            fce: |params| params.iter().sum(),
            params_validation: |count| count > 0,
        },
        Function {
            representation: "average",
            fce: |params| params.iter().sum::<f64>() / params.len() as f64,
            params_validation: |count| count > 0,
        },
        Function {
            representation: "median",
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
        },
        Function {
            representation: "count",
            fce: |params| params.len() as f64,
            params_validation: |count| count > 0,
        },
    ] {
        functions.insert(function.representation, function);
    }

    functions
});

pub static FUNCTION_NAMES: Lazy<Vec<&str>> = Lazy::new(|| {
    let mut names = Vec::new();
    for (k, _) in FUNCTIONS.iter() {
        names.push(*k);
    }

    // sorted reversed so sqrt is before sqr
    names.sort();
    names.reverse();

    names
});
