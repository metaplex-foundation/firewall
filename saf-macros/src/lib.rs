use crate::error::derive_error;
use proc_macro::TokenStream;

extern crate proc_macro;
mod error;

#[proc_macro_derive(ProgramErrorCode)]
pub fn derive_error_codes(input: TokenStream) -> TokenStream {
    derive_error(input)
}
