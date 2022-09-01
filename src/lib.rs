use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ActixHeader)]
pub fn actix_header(token: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(token);
    let ident_str = ident.to_string();
    let output = quote!(
        use std::str::FromStr;
        impl TryIntoHeaderValue for #ident {
            type Error = InvalidHeaderValue;
            fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
                let s: String = self.into();
                HeaderValue::from_str(&s)
            }
        }

        impl ParseHeader for #ident {
            fn name() -> HeaderName {
                HeaderName::from_str(#ident_str).unwrap()
            }
            fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
                let s = msg.headers().get(Self::name()).ok_or(ParseError::Header)?.to_str().map_err(|e| {
                    error!("{}", e);
                    ParseError::Header
                })?;
                Ok(Self::from(s.to_owned()))
            }
        }
    );
    output.into()
}
