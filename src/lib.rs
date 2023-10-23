//! Additional "fun" extractors for macroex.

use macroex::*;

/// Parses an angle.
/// # Cases
/// * `45 degrees` or `45.0 deg`
/// * `1.2 rad` or `0 radians`
/// * `pi`
/// * `pi/2` or  `pi/2.0`
/// * `2 pi` or  `2.0 pi`
/// * `2/3 pi` or  `2.0 / 3.0 pi`
/// 
/// To use with from_one, enclose in brackets `[]`
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Angle(pub f32);

impl FromMacro for Angle {
    fn from_one(tt: proc_macro2::TokenTree) -> Result<Self, Error> {
        let Bracketed(tokens) = Bracketed::from_one(tt)?;
        Self::from_many(tokens)
    }

    fn from_many(tokens: proc_macro2::TokenStream) -> Result<Self, Error> {
        let mut iter = tokens.into_iter();
        ident_validator!(PI "pi");
        ident_validator!(Deg "deg");
        match iter.extract()? {
            Either::A(PI) => {
                if let Ok(PunctOf::<'/'>) = iter.extract(){
                    let Number(numer) = iter.extract()?;
                    let EndOfStream = iter.extract()?;
                    Ok(Self(std::f32::consts::PI / numer))
                } else {
                    Ok(Self(std::f32::consts::PI))
                }
            },
            Either::B(Number(value)) => {
                match iter.extract()? {
                    Either::A(IdentString(s)) => {
                        match s.as_str() {
                            "pi" => (),
                            "rad" | "radians" => (),
                            "deg" | "degrees" => (),
                        }
                    },
                    Either::B(PunctOf::<'/'>) => (),
                }
            },
        }
    }
}


/// Parses a Rectangle.
/// 
/// Accepts 2 of 
/// `min`, `center`, `max` and `dim`.
/// 
/// Returns `min` and `dim`.
pub struct Rectangle(pub [f32;2], pub [f32;2]);


/// Parses an Rectangle.
/// 
/// Accepts 2 of 
/// `bottomleft`, `center`, `topright` and `dimension`.
/// 
/// Returns `bottomleft` and `dimension`.
pub struct RectBLTR(pub [f32;2], pub [f32;2]);

/// Parses an Rectangle.
/// 
/// Accepts 2 of 
/// `topleft`, `center`, `bottomright` and `dimension`.
/// 
/// Returns `topleft` and `dimension`.
pub struct RectTLBR(pub [f32;2], pub [f32;2]);



/// Parses an RGBA Color.
/// 
/// See `colorthis` for documentations.
pub struct Rgba<T>(T);