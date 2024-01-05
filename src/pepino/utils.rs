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
