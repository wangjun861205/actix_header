use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn actix_header(attr: TokenStream, token: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token);
    let ident = input.ident.clone();
    let name = attr.to_string();
    let output = quote!(
        #input

        use std::str::FromStr;
        use actix_web::error::ParseError;

        impl TryIntoHeaderValue for #ident {
            type Error = InvalidHeaderValue;
            fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
                let s: String = self.into();
                HeaderValue::from_str(&s)
            }

        }
        impl ParseHeader for #ident {
            fn name() -> HeaderName {
                HeaderName::from_str(#name).unwrap()
            }
            fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
                let s = msg.headers().get(Self::name()).ok_or(ParseError::Header)?.to_str().map_err(|e| {
                    ParseError::Header
                })?;
                Ok(Self::from(s.to_owned()))
            }
        }
    );
    output.into()
}
