
use macroex::{*, proc_macro2::*};
/// Parses an angle.
/// # Cases
/// ## from_one
/// * `f32`
/// * `pi`
/// * `[from_many]`

/// ## from_many

/// * `45 degrees` or `45.0 deg`
/// * `1.2 rad` or `0 radians`
/// * `pi`
/// * `pi/2` or  `pi/2.0`
/// * `2 pi` or  `2.0 pi`
/// * `2/3 pi` or  `2.0 / 3.0 pi`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle(pub f32);

pub const PI: f32 = std::f32::consts::PI;
ident_validator!(LitPi "pi");
impl FromMacro for Angle {

    fn from_one(tt: proc_macro2::TokenTree) -> Result<Self, Error> {
        match Either3::from_one(tt)? {
            Either3::A(Number(f)) => Ok(Self(f)),
            Either3::B(LitPi) => Ok(Self(PI)),
            Either3::C(Bracketed(tokens)) => Self::from_many(tokens)
        }
    }

    fn from_many(tokens: proc_macro2::TokenStream) -> Result<Self, Error> {
        let mut iter = tokens.into_iter();
        match iter.extract()? {
            Either3::A(LitPi) => {
                if let Ok(PunctOf::<'/'>) = iter.extract(){
                    let Number::<f32>(numer) = iter.extract()?;
                    let EndOfStream = iter.extract()?;
                    Ok(Self(PI / numer))
                } else {
                    let EndOfStream = iter.extract()?;
                    Ok(Self(PI))
                }
            },
            Either3::B(Number(value)) => {
                match iter.extract()? {
                    OrEndOfStream(Some(Either::A(Spanned(span, IdentString(s))))) => {
                        match s.as_str() {
                            "pi" => Ok(Self(value * PI)),
                            "rad" | "radians" => Ok(Self(value)),
                            "deg" | "degrees" => Ok(Self(value * PI / 180.0)),
                            _ => bail!(span, r#"Expected "pi", "rad", "radians", "deg" or "degrees", found {}"#, s)
                        }
                    },
                    OrEndOfStream(Some(Either::B(PunctOf::<'/'>))) => {
                        let Number::<f32>(numer) = iter.extract()?;
                        let LitPi = iter.extract()?;
                        Ok(Self(value / numer * PI))
                    },
                    OrEndOfStream(None) => {
                        Ok(Self(value))
                    }
                }
            },
            Either3::C(PunctOf::<'-'>) => {
                let angle = Self::from_many(iter.collect())?;
                Ok(Self(-angle.0))
            }
        }
    }

    /// The brace case will be treated as an expression.
    fn peek(tt: &proc_macro2::TokenTree) -> bool {
        match tt {
            proc_macro2::TokenTree::Group(g) => {
                g.delimiter() != Delimiter::Brace
            },
            _ => true,
        }
    }
}

impl quote::ToTokens for Angle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}
