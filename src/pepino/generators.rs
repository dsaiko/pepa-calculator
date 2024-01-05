use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Generator {
    pub fce_name: String,
    pub fce: fn() -> f64,
}

pub static GENERATORS: Lazy<HashMap<String, Generator>> = Lazy::new(|| {
    let mut generators = HashMap::new();

    for generator in [
        Generator {
            fce_name: "random()".to_owned(),
            fce: fastrand::f64,
        },
        Generator {
            fce_name: "timestamp()".to_owned(),
            fce: || {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as f64
            },
        },
    ] {
        let name = generator.fce_name.to_ascii_uppercase().replace("()", "");
        generators.insert(name, generator);
    }

    generators
});
