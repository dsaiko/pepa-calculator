use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static CONSTANTS: Lazy<HashMap<&str, f64>> =
    Lazy::new(|| HashMap::from([("E", std::f64::consts::E), ("PI", std::f64::consts::PI)]));
