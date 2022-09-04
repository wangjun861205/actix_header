use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn actix_header(attr: TokenStream, token: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token);
    let ident = input.ident.clone();
    let name = attr.to_string().replace("\"", "");
    let output = quote!(
        #input

        use std::str::FromStr;

        impl actix_web::http::header::TryIntoHeaderValue for #ident {
            type Error = actix_web::http::header::InvalidHeaderValue;

            fn try_into_value(self) -> Result<actix_web::http::header::HeaderValue, Self::Error> {

                let s: String = self.into();
                actix_web::http::header::HeaderValue::from_str(&s)
            }

        }
        impl actix_web::http::header::Header for #ident {
            fn name() -> actix_web::http::header::HeaderName {
                actix_web::http::header::HeaderName::from_str(#name).unwrap()
            }

            fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError> {
                let s = msg.headers().get(#name).ok_or(actix_web::error::ParseError::Header)?.to_str().map_err(|e| {
                    actix_web::error::ParseError::Header
                })?;
                Ok(Self::from(s.to_owned()))
            }
        }
    );
    output.into()
}
