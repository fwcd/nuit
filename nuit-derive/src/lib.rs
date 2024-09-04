mod approx_eq;
mod bind;

use proc_macro::TokenStream;

#[proc_macro_derive(ApproxEq)]
pub fn derive_approx_eq(input: TokenStream) -> TokenStream {
    approx_eq::derive(input)
}

#[proc_macro_derive(Bind)]
pub fn derive_bind(input: TokenStream) -> TokenStream {
    bind::derive(input)
}
