use rust_decimal_macros::dec;
use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::units::angle::Angle;
use crate::units::{Length, Mass, Temperature, Time};
use crate::{string, Calculator, Decimal, NumericExpression};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum Unit {
    Temperature(Temperature),
    Time(Time),
    Length(Length),
    Mass(Mass),
    Angle(Angle),
}

pub struct Abbreviations {
    pub case_sensitive: HashMap<String, Unit>,
    pub case_insensitive: HashMap<String, Unit>,
}

impl Unit {
    fn abbreviations(&self) -> Abbreviations {
        match self {
            Unit::Temperature(_) => Temperature::abbreviations(),
            Unit::Time(_) => Time::abbreviations(),
            Unit::Length(_) => Length::abbreviations(),
            Unit::Mass(_) => Mass::abbreviations(),
            Unit::Angle(_) => Angle::abbreviations(),
        }
    }

    pub fn from_string(name: &str) -> Vec<Unit> {
        let mut res = Vec::new();

        for u in Unit::iter() {
            let abbreviations = u.abbreviations();
            if let Some(u) = abbreviations.case_sensitive.get(&string!(name)) {
                res.push(*u);
            }
        }

        if !res.is_empty() {
            return res;
        }

        for u in Unit::iter() {
            let abbreviations = u.abbreviations();
            if let Some(u) = abbreviations.case_insensitive.get(&name.to_lowercase()) {
                res.push(*u);
            }
        }

        res
    }

    pub fn conversion(&self, v: &Decimal, to: &Unit) -> Option<Decimal> {
        match self {
            Unit::Temperature(from) => match to {
                Unit::Temperature(to) => Some(to.from_reference_unit(from.to_reference_unit(*v))),
                _ => None,
            },
            Unit::Time(from) => match to {
                Unit::Time(to) => {
                    Some(v * from.reference_unit_multiplier() / to.reference_unit_multiplier())
                }
                _ => None,
            },
            Unit::Length(from) => match to {
                Unit::Length(to) => {
                    Some(v * from.reference_unit_multiplier() / to.reference_unit_multiplier())
                }
                _ => None,
            },
            Unit::Mass(from) => match to {
                Unit::Mass(to) => {
                    Some(v * from.reference_unit_multiplier() / to.reference_unit_multiplier())
                }
                _ => None,
            },
            Unit::Angle(from) => match to {
                Unit::Angle(to) => {
                    Some(v * from.reference_unit_multiplier() / to.reference_unit_multiplier())
                }
                _ => None,
            },
        }
    }

    pub fn to_string_with_plural(&self, n: &Decimal) -> String {
        match self {
            Unit::Temperature(t) => t.to_string_with_plural(n),
            Unit::Time(t) => t.to_string_with_plural(n),
            Unit::Length(l) => l.to_string_with_plural(n),
            Unit::Mass(m) => m.to_string_with_plural(n),
            Unit::Angle(a) => a.to_string_with_plural(n),
        }
    }
}

#[cfg(test)]
pub fn test_units(test: &str, res: &[(Decimal, Option<Unit>)]) {
    let mut computer = Calculator::default();
    let statement = computer.compute(test).unwrap();

    if let Err(e) = &statement.expression {
        panic!("Error in expression: '{:?}': {:?}", test, e);
    }

    match &statement.result {
        None => panic!("No result for: '{:?}'", test),
        Some(Err(e)) => panic!("Error in computation: '{:?}': {:?}", test, e),
        Some(Ok(n)) => {
            let t = NumericExpression::with_multiple_units(res.to_vec());
            let mut ok = true;

            let mut v1 = t.values();
            let mut v2 = n.values();

            if v1.len() != v2.len() {
                ok = false;
            } else {
                for i in 0..v1.len() {
                    if v1[i].1 != v2[i].1 {
                        ok = false;
                    }

                    let n1 = (v1[i].0 * dec!(100)).round() / dec!(100);
                    let n2 = (v2[i].0 * dec!(100)).round() / dec!(100);

                    v1[i].0 = n1;
                    v2[i].0 = n2;

                    if n1 != n2 {
                        ok = false;
                    }
                }
            }

            if !ok {
                panic!("{:?}: {:?} != {:?}", test, v1, v2);
            }
        }
    }
}
