use std::collections::HashMap;
use std::sync::OnceLock;

use rust_decimal_macros::dec;

use crate::Decimal;

pub const C: Decimal = dec!(299_792_458);

pub fn constants() -> &'static HashMap<&'static str, Decimal> {
    static MEM: OnceLock<HashMap<&'static str, Decimal>> = OnceLock::new();
    MEM.get_or_init(|| {
        HashMap::from([
            ("E", Decimal::E),
            ("PI", Decimal::PI),
            // speed of light
            ("C", C),
        ])
    })
}
