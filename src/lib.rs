//! Additional "fun" extractors for macroex.

use macroex::{proc_macro2::{TokenStream, TokenTree, Delimiter}, FromMacro};
use quote::ToTokens;

mod angle;
mod color;

/// This replaces the idiom `Option<Either<CurlyBraced<TokenStream>, T>>` commonly
/// used in parsing contents that may be expressions.
#[derive(Debug, Clone, Default)]
pub enum MaybeExpr<T>{
    #[default]
    None,
    Value(T),
    Expr(TokenStream),
}


impl<T: FromMacro> FromMacro for MaybeExpr<T> {
    fn from_one(tt: macroex::proc_macro2::TokenTree) -> Result<Self, macroex::Error> {
        match tt {
            TokenTree::Group(ref g) if g.delimiter() == Delimiter::Brace => {
                Ok(Self::Expr(g.stream()))
            },
            tt => Ok(Self::Value(T::from_one(tt)?))
        }
    }
}


impl<T: ToTokens> quote::ToTokens for MaybeExpr<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            MaybeExpr::None => panic!("Cannot format None."),
            MaybeExpr::Value(v) => v.to_tokens(tokens),
            MaybeExpr::Expr(t) => t.to_tokens(tokens),
        }
    }
}