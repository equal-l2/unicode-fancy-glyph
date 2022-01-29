#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use unicode_fancy_glyph as lib;

fn parse_into_glyph<S: AsRef<str>>(value: S) -> Option<lib::Glyph> {
    use lib::Glyph;
    Some(match value.as_ref() {
        "b" => Glyph::Bold,
        "i" => Glyph::Italic,
        "bi" => Glyph::BoldItalic,
        "sc" => Glyph::Script,
        "bs" => Glyph::BoldScript,
        "f" => Glyph::Fraktur,
        "d" => Glyph::DoubleStruck,
        "bf" => Glyph::BoldFraktur,
        "ss" => Glyph::SansSerif,
        "ssb" => Glyph::SansSerifBold,
        "ssi" => Glyph::SansSerifItalic,
        "ssbi" => Glyph::SansSerifBoldItalic,
        "m" => Glyph::Monospace,
        _ => return None,
    })
}

fn main() {
    let mut args = std::env::args();
    args.next().expect("argv[0] must exist on POSIX platforms");
    let glyph_opt: Option<lib::Glyph> = args.next().and_then(parse_into_glyph);
    let input_opt = args.next();

    let args_opt = glyph_opt.zip(input_opt);

    if let Some((glyph, input)) = args_opt {
        let read_stdin = input == "-";
        let s = if read_stdin {
            let mut s = String::new();
            std::io::Read::read_to_string(&mut std::io::stdin(), &mut s)
                .expect("stdin should be always readable");
            s
        } else {
            input
        };
        let out = lib::convert(glyph, s);
        if read_stdin {
            print!("{out}");
        } else {
            println!("{out}");
        }
    } else {
        eprintln!("ufg [glyph] [input]");
        eprintln!("glyph: b, i, bi, sc, bs, f, d, bf, ss, ssb, ssi, ssbi, m");
        eprintln!("input: string or \"-\" (read from stdin)");
        std::process::exit(1);
    }
}
