use macroex::{FromMacro, Either4, HexNumber, proc_macro2::{Span, Ident, TokenTree}, Error, bail, NumberList, Either, Splat};

fn hex(a: u8, span: Span) -> Result<u8, Error> {
    Ok(match a {
        b'0'..= b'9' => a - b'0',
        b'a'..= b'z' => a - b'a' + 10,
        b'A'..= b'Z' => a - b'A' + 10,
        _ => bail!(span, "Not a valid hexadecial number.")
    })
}

fn hex2(a: u8, b: u8, span: Span) -> Result<u8, Error> {
    Ok((hex(a, span)? << 4) + hex(b, span)?)
}


fn parse_slice(lit: &[u8], span: Span) -> Result<[u8; 4], Error>{
    Ok(match lit.len() {
        3 => [
            hex(lit[0], span)? * 17,
            hex(lit[1], span)? * 17,
            hex(lit[2], span)? * 17,
            255
        ],
        4 => [
            hex(lit[0], span)? * 17,
            hex(lit[1], span)? * 17,
            hex(lit[2], span)? * 17,
            hex(lit[3], span)? * 17,
        ],
        6 => [
            hex2(lit[0], lit[1], span)?,
            hex2(lit[2], lit[3], span)?,
            hex2(lit[4], lit[5], span)?,
            255
        ],
        8 => [
            hex2(lit[0], lit[1], span)?,
            hex2(lit[2], lit[3], span)?,
            hex2(lit[4], lit[5], span)?,
            hex2(lit[6], lit[7], span)?,
        ],
        _ => bail!(span, "Invalid color syntax, must be of length 3, 4, 6 or 8."),
    })
}

fn f2i(floats: [f32; 4]) -> [u8; 4] {
    [
        (floats[0] * 255.0) as u8,
        (floats[1] * 255.0) as u8,
        (floats[2] * 255.0) as u8,
        (floats[3] * 255.0) as u8,
    ]
}

fn smart_i2f(floats: [u8; 4]) -> [f32; 4] {
    if floats.iter().all(|x| *x == 0 || *x == 1) {
        [
            floats[0] as f32,
            floats[1] as f32,
            floats[2] as f32,
            floats[3] as f32,
        ]
    } else {
        [
            floats[0] as f32 / 255.0,
            floats[1] as f32 / 255.0,
            floats[2] as f32 / 255.0,
            floats[3] as f32 / 255.0,
        ]
    }
}

fn i2f(floats: [u8; 4]) -> [f32; 4] {
    [
        floats[0] as f32 / 255.0,
        floats[1] as f32 / 255.0,
        floats[2] as f32 / 255.0,
        floats[3] as f32 / 255.0,
    ]
}

/// Parses an RGBA Color.
///
/// # Schema
/// * Bracketed numbers: `[0.3, 0.72, 0.98]`, `[124, 54, 87, 255]`
/// * Repeat syntax: `[0.3; 3]`, `[0.7; 4]`
/// * Hex strings: `"AABBCC"`, `"AABBCCFF"`, `"#AABBCC"`, `"#AABBCCFF"`
/// * Hex number literals: `0xAABBCC`, `0xAABBCCFF`
/// * CSS color names: `Red`, `Blue`
/// * TailwindCSS color names: `Red100`, `Sky400`
///
/// # Conversion
/// Ints are in `0..=255`, floats are in `0.0..=1.0`.
///
/// When parsing to u8, if any value is a float, the color is considered in range `0.0..=1.0`
///
/// When parsing to float, if all values are ints and any of them is`>= 2`, the color is considered to be in range `0.0..=255.0`.
pub struct Rgba<T>(pub T);

fn parse_color_name(name: &str, span: Span) -> Result<[u8; 4], Error>{
    if let Some(num) = name.find(|x| ('0'..='9').contains(&x)) {
        let (color, right) = name.split_at(num);
        if let Ok(index) = right.parse() {
            if let Some(color) = parse_color::parse_tailwind(color, index){
                Ok(color)
            } else {
                bail!(span, "Invalid tailwind color {}-{}", color, index)
            }
        } else {
            bail!(span, "Failed to parse color \"{}\"", name)
        }
    } else {
        match parse_color::parse(&name){
            Some(x) => Ok(x),
            None => bail!(span, "Failed to parse color \"{}\"", name),
        }
    }
}

fn padi(arr: [u8; 3]) -> [u8; 4] {
    [arr[0], arr[1], arr[2], 255]
}

fn padif(arr: [u8; 3]) -> [u8; 4] {
    if arr.iter().all(|x| *x == 0 || *x == 1) {
        [arr[0], arr[1], arr[2], 1]
    } else {
        [arr[0], arr[1], arr[2], 255]
    }
}

fn padf(arr: [f32; 3]) -> [f32; 4] {
    [arr[0], arr[1], arr[2], 1.0]
}

impl FromMacro for Rgba<[u8; 4]> {
    fn from_one(tt: macroex::proc_macro2::TokenTree) -> Result<Self, macroex::Error> {
        let span = tt.span();
        match Either4::from_one(tt)? {
            Either4::A(Splat(group)) => {
                let tt = TokenTree::Group(group);
                match Either4::from_one(tt)? {
                    Either4::A(ints) => Ok(Self(padi(ints))),
                    Either4::B(ints) => Ok(Self(ints)),
                    Either4::C(NumberList(floats)) => Ok(Self(f2i(padf(floats)))),
                    Either4::D(NumberList(floats)) => Ok(Self(f2i(floats))),
                }
            },
            Either4::B(string) => {
                let string: String = string;
                if string.starts_with('#') {
                    Ok(Self(parse_slice(&string.as_bytes()[1..], span)?))
                } else {
                    Ok(Self(parse_slice(&string.as_bytes(), span)?))
                }
            },
            Either4::C(ident) => {
                let ident: Ident = ident;
                let name = ident.to_string();
                Ok(Self(parse_color_name(&name, span)?))
            },
            Either4::D(HexNumber(_, hex)) => {
                Ok(Self(parse_slice(&hex.as_bytes(), span)?))
            },
        }
    }
}


impl FromMacro for Rgba<[f32; 4]> {
    fn from_one(tt: macroex::proc_macro2::TokenTree) -> Result<Self, macroex::Error> {
        let span = tt.span();
        match Either4::from_one(tt)? {
            Either4::A(Splat(group)) => {
                let tt = TokenTree::Group(group);
                match Either4::from_one(tt)? {
                    Either4::A(ints) => Ok(Self(smart_i2f(padif(ints)))),
                    Either4::B(ints) => Ok(Self(smart_i2f(ints))),
                    Either4::C(NumberList(floats)) => Ok(Self(padf(floats))),
                    Either4::D(NumberList(floats)) => Ok(Self(floats)),
                }
            },
            Either4::B(string) => {
                let string: String = string;
                if string.starts_with('#') {
                    Ok(Self(i2f(parse_slice(&string.as_bytes()[1..], span)?)))
                } else {
                    Ok(Self(i2f(parse_slice(&string.as_bytes(), span)?)))
                }
            },
            Either4::C(ident) => {
                let ident: Ident = ident;
                let name = ident.to_string();
                Ok(Self(i2f(parse_color_name(&name, span)?)))
            },
            Either4::D(HexNumber(_, hex)) => {
                Ok(Self(i2f(parse_slice(&hex.as_bytes(), span)?)))
            },
        }
    }
}
