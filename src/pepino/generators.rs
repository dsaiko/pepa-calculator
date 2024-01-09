use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Decimal, string};

#[derive(Debug, Clone)]
pub struct Generator {
    pub fce_name: String,
    pub fce: fn() -> Decimal,
}

pub fn generators() -> &'static HashMap<String, Generator> {
    static MEM: OnceLock<HashMap<String, Generator>> = OnceLock::new();
    MEM.get_or_init(|| {
        let mut generators = HashMap::new();

        for generator in [
            Generator {
                fce_name: string!("random()"),
                fce: || Decimal::from_f64_retain(fastrand::f64()).unwrap(),
            },
            Generator {
                fce_name: string!("timestamp()"),
                fce: || {
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        .into()
                },
            },
        ] {
            let name = generator.fce_name.to_ascii_uppercase().replace("()", "");

            generators.insert(name, generator);
        }

        generators
    })
}
