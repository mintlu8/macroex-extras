//! Additional "fun" extractors for macroex.

use macroex::{proc_macro2::{TokenStream, TokenTree, Delimiter}, FromMacro, StreamExtract, All};
use quote::ToTokens;

mod angle;
mod color;

pub use angle::Angle;
pub use color::Rgba;

/// This replaces the idiom `Option<Either<CurlyBraced<TokenStream>, T>>` commonly
/// used in parsing contents that may be expressions.
#[derive(Debug, Clone, Default)]
pub enum MaybeExpr<T, TExpr=TokenStream>{
    #[default]
    None,
    Value(T),
    Expr(TExpr),
}

impl<T, TExpr> MaybeExpr<T, TExpr> {
    pub fn is_some(&self) -> bool {
        !matches!(self, MaybeExpr::None)
    }
    
    pub fn is_none(&self) -> bool {
        matches!(self, MaybeExpr::None)
    }

    pub fn get(&self) -> Option<ValueOrExpr<T, TExpr>> {
        match self {
            MaybeExpr::None => None,
            MaybeExpr::Expr(e) => Some(ValueOrExpr::Expr(e)),
            MaybeExpr::Value(v) => Some(ValueOrExpr::Value(v)),
        }
    }
}


impl<T: FromMacro, TExpr: FromMacro> FromMacro for MaybeExpr<T, TExpr> {
    fn from_one(tt: macroex::proc_macro2::TokenTree) -> Result<Self, macroex::Error> {
        match tt {
            TokenTree::Group(ref g) if g.delimiter() == Delimiter::Brace => {
                let All(expr) = g.stream().into_iter().extract()?;
                Ok(Self::Expr(expr))
            },
            tt => Ok(Self::Value(T::from_one(tt)?))
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueOrExpr<'t, T, TExpr=TokenStream>{
    Value(&'t T),
    Expr(&'t TExpr),
}


impl<T: ToTokens, TExpr: ToTokens> quote::ToTokens for ValueOrExpr<'_, T, TExpr> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ValueOrExpr::Value(v) => v.to_tokens(tokens),
            ValueOrExpr::Expr(t) => t.to_tokens(tokens),
        }
    }
}
