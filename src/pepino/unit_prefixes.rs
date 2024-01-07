use std::fmt::{Display, Formatter};

use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;

use crate::Decimal;

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
pub enum UnitPrefix {
    Quetta,
    Ronna,
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deka,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
    Ronto,
    Quecto,
}

impl UnitPrefix {
    pub fn multiplier(&self) -> Decimal {
        match self {
            UnitPrefix::Quetta => dec!(10).powu(30),
            UnitPrefix::Ronna => dec!(10).powu(27),
            UnitPrefix::Yotta => dec!(10).powu(24),
            UnitPrefix::Zetta => dec!(10).powu(21),
            UnitPrefix::Exa => dec!(10).powu(18),
            UnitPrefix::Peta => dec!(10).powu(15),
            UnitPrefix::Tera => dec!(10).powu(12),
            UnitPrefix::Giga => dec!(10).powu(9),
            UnitPrefix::Mega => dec!(10).powu(6),
            UnitPrefix::Kilo => dec!(10).powu(3),
            UnitPrefix::Hecto => dec!(10).powu(2),
            UnitPrefix::Deka => dec!(10).powu(1),

            UnitPrefix::Deci => dec!(10).powi(-1),
            UnitPrefix::Centi => dec!(10).powi(-2),
            UnitPrefix::Milli => dec!(10).powi(-3),
            UnitPrefix::Micro => dec!(10).powi(-6),
            UnitPrefix::Nano => dec!(10).powi(-9),
            UnitPrefix::Pico => dec!(10).powi(-12),
            UnitPrefix::Femto => dec!(10).powi(-15),
            UnitPrefix::Atto => dec!(10).powi(-18),
            UnitPrefix::Zepto => dec!(10).powi(-21),
            UnitPrefix::Yocto => dec!(10).powi(-24),
            UnitPrefix::Ronto => dec!(10).powi(-27),
            UnitPrefix::Quecto => dec!(10).powi(-30),
        }
    }
}

impl Display for UnitPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnitPrefix::Quetta => "quetta",
                UnitPrefix::Ronna => "ronna",
                UnitPrefix::Yotta => "yotta",
                UnitPrefix::Zetta => "zetta",
                UnitPrefix::Exa => "exa",
                UnitPrefix::Peta => "peta",
                UnitPrefix::Tera => "tera",
                UnitPrefix::Giga => "giga",
                UnitPrefix::Mega => "mega",
                UnitPrefix::Kilo => "kilo",
                UnitPrefix::Hecto => "hecto",
                UnitPrefix::Deka => "deka",
                UnitPrefix::Deci => "deci",
                UnitPrefix::Centi => "centi",
                UnitPrefix::Milli => "milli",
                UnitPrefix::Micro => "micro",
                UnitPrefix::Nano => "nano",
                UnitPrefix::Pico => "pico",
                UnitPrefix::Femto => "femto",
                UnitPrefix::Atto => "atto",
                UnitPrefix::Zepto => "zepto",
                UnitPrefix::Yocto => "yocto",
                UnitPrefix::Ronto => "ronto",
                UnitPrefix::Quecto => "quecto",
            }
        )
    }
}
