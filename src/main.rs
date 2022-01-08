#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
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

enum Glyph {
    Bold,
    Italic,
    BoldItalic,
    Script,
    BoldScript,
    Fraktur,
    DoubleStruck,
    BoldFraktur,
    SansSerif,
    SansSerifBold,
    SansSerifItalic,
    SansSerifBoldItalic,
    Monospace,
}

impl std::str::FromStr for Glyph {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "b" => Self::Bold,
            "i" => Self::Italic,
            "bi" => Self::BoldItalic,
            "sc" => Self::Script,
            "bs" => Self::BoldScript,
            "f" => Self::Fraktur,
            "d" => Self::DoubleStruck,
            "bf" => Self::BoldFraktur,
            "ss" => Self::SansSerif,
            "ssb" => Self::SansSerifBold,
            "ssi" => Self::SansSerifItalic,
            "ssbi" => Self::SansSerifBoldItalic,
            "m" => Self::Monospace,
            _ => return Err(()),
        })
    }
}

impl From<Glyph> for Rule {
    fn from(value: Glyph) -> Self {
        match value {
            Glyph::Bold => Self::new(0x1d400),
            Glyph::Italic => Self::with_except(0x1d434, [('h', '\u{210e}')]),
            Glyph::BoldItalic => Self::new(0x1d468),
            Glyph::Script => Self::with_except(
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
            Glyph::BoldScript => Self::new(0x1d4d0),
            Glyph::Fraktur => Self::with_except(
                0x1d504,
                [
                    ('C', '\u{212d}'),
                    ('H', '\u{210c}'),
                    ('I', '\u{2111}'),
                    ('R', '\u{211c}'),
                    ('Z', '\u{2128}'),
                ],
            ),
            Glyph::DoubleStruck => Self::with_except(
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
            Glyph::BoldFraktur => Self::new(0x1d56c),
            Glyph::SansSerif => Self::new(0x1d5a0),
            Glyph::SansSerifBold => Self::new(0x1d5d4),
            Glyph::SansSerifItalic => Self::new(0x1d608),
            Glyph::SansSerifBoldItalic => Self::new(0x1d63c),
            Glyph::Monospace => Self::new(0x1d670),
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

fn convert_glyphs<T: AsRef<str>>(glyph: Glyph, input: T) -> String {
    let Rule { start, except } = Rule::from(glyph);
    let input = input.as_ref();
    let conv = |c| {
        try_diff(c).map_or(c, |diff| unsafe {
            std::char::from_u32_unchecked(start + diff)
        })
    };

    let chars = input.chars();
    if let Some(map) = except {
        chars
            .map(|ch| map.get(&ch).copied().unwrap_or_else(|| conv(ch)))
            .collect()
    } else {
        chars.map(conv).collect()
    }
}

fn main() {
    let glyph_opt: Option<Glyph> = std::env::args().nth(1).and_then(|s| s.parse().ok());

    if let Some(glyph) = glyph_opt {
        let mut s = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut s).unwrap();
        let out = convert_glyphs(glyph, s);
        print!("{}", out);
    } else {
        eprintln!(
            r#"expected one of "b", "i", "bi", "sc", "bs", "f", "d", "bf", "ss", "ssb", "ssi", "ssbi", "m""#
        );
    }
}

#[test]
fn test_convert() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let res = convert_glyphs(Glyph::Bold, input);
    assert_eq!(res, "𝐀𝐁𝐂𝐃𝐄𝐅𝐆𝐇𝐈𝐉𝐊𝐋𝐌𝐍𝐎𝐏𝐐𝐑𝐒𝐓𝐔𝐕𝐖𝐗𝐘𝐙𝐚𝐛𝐜𝐝𝐞𝐟𝐠𝐡𝐢𝐣𝐤𝐥𝐦𝐧𝐨𝐩𝐪𝐫𝐬𝐭𝐮𝐯𝐰𝐱𝐲𝐳");
    let res = convert_glyphs(Glyph::Italic, input);
    assert_eq!(res, "𝐴𝐵𝐶𝐷𝐸𝐹𝐺𝐻𝐼𝐽𝐾𝐿𝑀𝑁𝑂𝑃𝑄𝑅𝑆𝑇𝑈𝑉𝑊𝑋𝑌𝑍𝑎𝑏𝑐𝑑𝑒𝑓𝑔ℎ𝑖𝑗𝑘𝑙𝑚𝑛𝑜𝑝𝑞𝑟𝑠𝑡𝑢𝑣𝑤𝑥𝑦𝑧");
    let res = convert_glyphs(Glyph::BoldItalic, input);
    assert_eq!(res, "𝑨𝑩𝑪𝑫𝑬𝑭𝑮𝑯𝑰𝑱𝑲𝑳𝑴𝑵𝑶𝑷𝑸𝑹𝑺𝑻𝑼𝑽𝑾𝑿𝒀𝒁𝒂𝒃𝒄𝒅𝒆𝒇𝒈𝒉𝒊𝒋𝒌𝒍𝒎𝒏𝒐𝒑𝒒𝒓𝒔𝒕𝒖𝒗𝒘𝒙𝒚𝒛");
    let res = convert_glyphs(Glyph::Script, input);
    assert_eq!(res, "𝒜ℬ𝒞𝒟ℰℱ𝒢ℋℐ𝒥𝒦ℒℳ𝒩𝒪𝒫𝒬ℛ𝒮𝒯𝒰𝒱𝒲𝒳𝒴𝒵𝒶𝒷𝒸𝒹ℯ𝒻ℊ𝒽𝒾𝒿𝓀𝓁𝓂𝓃ℴ𝓅𝓆𝓇𝓈𝓉𝓊𝓋𝓌𝓍𝓎𝓏");
    let res = convert_glyphs(Glyph::BoldScript, input);
    assert_eq!(res, "𝓐𝓑𝓒𝓓𝓔𝓕𝓖𝓗𝓘𝓙𝓚𝓛𝓜𝓝𝓞𝓟𝓠𝓡𝓢𝓣𝓤𝓥𝓦𝓧𝓨𝓩𝓪𝓫𝓬𝓭𝓮𝓯𝓰𝓱𝓲𝓳𝓴𝓵𝓶𝓷𝓸𝓹𝓺𝓻𝓼𝓽𝓾𝓿𝔀𝔁𝔂𝔃");
    let res = convert_glyphs(Glyph::Fraktur, input);
    assert_eq!(res, "𝔄𝔅ℭ𝔇𝔈𝔉𝔊ℌℑ𝔍𝔎𝔏𝔐𝔑𝔒𝔓𝔔ℜ𝔖𝔗𝔘𝔙𝔚𝔛𝔜ℨ𝔞𝔟𝔠𝔡𝔢𝔣𝔤𝔥𝔦𝔧𝔨𝔩𝔪𝔫𝔬𝔭𝔮𝔯𝔰𝔱𝔲𝔳𝔴𝔵𝔶𝔷");
    let res = convert_glyphs(Glyph::DoubleStruck, input);
    assert_eq!(res, "𝔸𝔹ℂ𝔻𝔼𝔽𝔾ℍ𝕀𝕁𝕂𝕃𝕄ℕ𝕆ℙℚℝ𝕊𝕋𝕌𝕍𝕎𝕏𝕐ℤ𝕒𝕓𝕔𝕕𝕖𝕗𝕘𝕙𝕚𝕛𝕜𝕝𝕞𝕟𝕠𝕡𝕢𝕣𝕤𝕥𝕦𝕧𝕨𝕩𝕪𝕫");
    let res = convert_glyphs(Glyph::BoldFraktur, input);
    assert_eq!(res, "𝕬𝕭𝕮𝕯𝕰𝕱𝕲𝕳𝕴𝕵𝕶𝕷𝕸𝕹𝕺𝕻𝕼𝕽𝕾𝕿𝖀𝖁𝖂𝖃𝖄𝖅𝖆𝖇𝖈𝖉𝖊𝖋𝖌𝖍𝖎𝖏𝖐𝖑𝖒𝖓𝖔𝖕𝖖𝖗𝖘𝖙𝖚𝖛𝖜𝖝𝖞𝖟");
    let res = convert_glyphs(Glyph::SansSerif, input);
    assert_eq!(res, "𝖠𝖡𝖢𝖣𝖤𝖥𝖦𝖧𝖨𝖩𝖪𝖫𝖬𝖭𝖮𝖯𝖰𝖱𝖲𝖳𝖴𝖵𝖶𝖷𝖸𝖹𝖺𝖻𝖼𝖽𝖾𝖿𝗀𝗁𝗂𝗃𝗄𝗅𝗆𝗇𝗈𝗉𝗊𝗋𝗌𝗍𝗎𝗏𝗐𝗑𝗒𝗓");
    let res = convert_glyphs(Glyph::SansSerifBold, input);
    assert_eq!(res, "𝗔𝗕𝗖𝗗𝗘𝗙𝗚𝗛𝗜𝗝𝗞𝗟𝗠𝗡𝗢𝗣𝗤𝗥𝗦𝗧𝗨𝗩𝗪𝗫𝗬𝗭𝗮𝗯𝗰𝗱𝗲𝗳𝗴𝗵𝗶𝗷𝗸𝗹𝗺𝗻𝗼𝗽𝗾𝗿𝘀𝘁𝘂𝘃𝘄𝘅𝘆𝘇");
    let res = convert_glyphs(Glyph::SansSerifItalic, input);
    assert_eq!(res, "𝘈𝘉𝘊𝘋𝘌𝘍𝘎𝘏𝘐𝘑𝘒𝘓𝘔𝘕𝘖𝘗𝘘𝘙𝘚𝘛𝘜𝘝𝘞𝘟𝘠𝘡𝘢𝘣𝘤𝘥𝘦𝘧𝘨𝘩𝘪𝘫𝘬𝘭𝘮𝘯𝘰𝘱𝘲𝘳𝘴𝘵𝘶𝘷𝘸𝘹𝘺𝘻");
    let res = convert_glyphs(Glyph::SansSerifBoldItalic, input);
    assert_eq!(res, "𝘼𝘽𝘾𝘿𝙀𝙁𝙂𝙃𝙄𝙅𝙆𝙇𝙈𝙉𝙊𝙋𝙌𝙍𝙎𝙏𝙐𝙑𝙒𝙓𝙔𝙕𝙖𝙗𝙘𝙙𝙚𝙛𝙜𝙝𝙞𝙟𝙠𝙡𝙢𝙣𝙤𝙥𝙦𝙧𝙨𝙩𝙪𝙫𝙬𝙭𝙮𝙯");
    let res = convert_glyphs(Glyph::Monospace, input);
    assert_eq!(res, "𝙰𝙱𝙲𝙳𝙴𝙵𝙶𝙷𝙸𝙹𝙺𝙻𝙼𝙽𝙾𝙿𝚀𝚁𝚂𝚃𝚄𝚅𝚆𝚇𝚈𝚉𝚊𝚋𝚌𝚍𝚎𝚏𝚐𝚑𝚒𝚓𝚔𝚕𝚖𝚗𝚘𝚙𝚚𝚛𝚜𝚝𝚞𝚟𝚠𝚡𝚢𝚣");
}

/*
#[test]
fn test_bench() {
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        test_convert();
    }
    let elapsed = start.elapsed().as_millis();
    assert_eq!(elapsed, 0);
}
*/
