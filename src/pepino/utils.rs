use rust_decimal::Decimal;

pub(super) fn split_string_by_comma(s: &str) -> Vec<String> {
    let s = s.trim();
    if s.is_empty() {
        return Vec::new();
    }

    if !s.contains(',') {
        return vec![s.to_owned()];
    }

    let mut res = Vec::new();
    let mut token = String::new();

    let mut pcount = 0;

    for c in s.chars() {
        match c {
            '(' => {
                pcount += 1;
                token.push(c);
            }
            ')' => {
                pcount -= 1;
                token.push(c);
            }
            ',' => {
                if pcount > 0 {
                    token.push(c);
                } else {
                    let t = token.trim();
                    if !t.is_empty() {
                        res.push(t.to_owned());
                    }
                    token.clear();
                }
            }
            _ => token.push(c),
        }
    }

    let t = token.trim();
    if !t.is_empty() {
        res.push(t.to_owned());
    }
    token.clear();

    res
}

pub(super) trait Pluralize {
    fn is_plural(&self) -> bool;
}

impl Pluralize for Decimal {
    fn is_plural(&self) -> bool {
        self.abs() != Decimal::ONE
    }
}

impl Pluralize for i64 {
    fn is_plural(&self) -> bool {
        self.abs() != 1
    }
}

#[macro_export]
macro_rules! pluralize {
    ($s:expr, $x:expr) => {
        if Pluralize::is_plural(&$x) {
            $s.to_owned() + "s"
        } else {
            $s.to_owned()
        }
    };
    ($s:expr, $p:expr, $x:expr) => {
        if Pluralize::is_plural(&$x) {
            $p.to_owned()
        } else {
            $s.to_owned()
        }
    };
}

#[macro_export]
macro_rules! string {
    ($s:expr) => {
        $s.to_owned()
    };
}
