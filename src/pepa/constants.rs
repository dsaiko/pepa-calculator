use std::collections::HashMap;
use std::sync::OnceLock;

use crate::Decimal;

pub fn constants() -> &'static HashMap<&'static str, Decimal> {
    static MEM: OnceLock<HashMap<&'static str, Decimal>> = OnceLock::new();
    MEM.get_or_init(|| HashMap::from([("E", Decimal::E), ("PI", Decimal::PI)]))
}
