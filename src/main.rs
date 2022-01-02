use std::collections::HashMap;

struct Rule {
    start: u32,
    except: Option<HashMap<char, char>>,
}

impl Rule {
    const fn new(start: u32) -> Self {
        Self {
            start,
            except: None,
        }
    }

    fn with_except(start: u32, map: impl Into<HashMap<char, char>>) -> Self {
        Self {
            start,
            except: Some(map.into()),
        }
    }
}

const fn try_diff(c: char) -> Option<u32> {
    match c {
        'A'..='Z' => Some(c as u32 - 'A' as u32),
        'a'..='z' => Some(c as u32 - 'a' as u32 + 26),
        _ => None,
    }
}

fn main() {
    let rules = HashMap::from([
        // bold
        ("b", Rule::new(0x1d400)),
        // italic
        ("i", Rule::with_except(0x1d434, [('h', '\u{210e}')])),
        // bold italic
        ("bi", Rule::new(0x1d468)),
        // script
        (
            "sc",
            Rule::with_except(
                0x1d49c,
                [
                    ('B', '\u{212c}'),
                    ('E', '\u{2130}'),
                    ('F', '\u{2131}'),
                    ('H', '\u{210b}'),
                    ('I', '\u{2110}'),
                    ('L', '\u{2112}'),
                    ('M', '\u{2133}'),
                    ('R', '\u{211b}'),
                    ('e', '\u{212f}'),
                    ('g', '\u{210a}'),
                    ('o', '\u{2134}'),
                ],
            ),
        ),
        // bold script
        ("bs", Rule::new(0x1d4d0)),
        // fraktur
        (
            "f",
            Rule::with_except(
                0x1d504,
                [
                    ('C', '\u{212d}'),
                    ('H', '\u{210c}'),
                    ('I', '\u{2111}'),
                    ('R', '\u{211c}'),
                    ('Z', '\u{2128}'),
                ],
            ),
        ),
        // double-struck
        (
            "d",
            Rule::with_except(
                0x1d538,
                [
                    ('C', '\u{2102}'),
                    ('H', '\u{210d}'),
                    ('N', '\u{2115}'),
                    ('P', '\u{2119}'),
                    ('Q', '\u{211a}'),
                    ('R', '\u{211d}'),
                    ('Z', '\u{2124}'),
                ],
            ),
        ),
        // bold fraktur
        ("bf", Rule::new(0x1d56c)),
        // sans-serif
        ("ss", Rule::new(0x1d5a0)),
        // sans-serif bold
        ("ssb", Rule::new(0x1d5d4)),
        // sans-serif italic
        ("ssi", Rule::new(0x1d608)),
        // sans-serif bold italic
        ("ssbi", Rule::new(0x1d63c)),
        // monospace
        ("m", Rule::new(0x1d670)),
    ]);

    let arg_error = || {
        eprintln!(
            r#"expected one of "b", "i", "bi", "sc", "bs", "f", "d", "bf", "ss", "ssb", "ssi", "ssbi", "m""#
        );
    };

    match std::env::args().nth(1) {
        Some(opt) => {
            let mut s = String::new();
            std::io::Read::read_to_string(&mut std::io::stdin(), &mut s).unwrap();

            let rule_opt = rules.get(opt.as_str());
            if let Some(rule) = rule_opt {
                let start = rule.start;
                let conv = |c| {
                    try_diff(c).map_or(c, |diff| unsafe {
                        std::char::from_u32_unchecked(start + diff)
                    })
                };

                let mut out = String::new();
                let map_opt = &rule.except;
                if let Some(map) = map_opt {
                    for ch in s.chars() {
                        out.push(map.get(&ch).copied().unwrap_or_else(|| conv(ch)));
                    }
                } else {
                    for ch in s.chars() {
                        out.push(conv(ch));
                    }
                }

                print!("{}", out);
            } else {
                arg_error();
            }
        }
        _ => {
            arg_error();
        }
    }
}
