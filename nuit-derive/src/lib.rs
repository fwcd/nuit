mod bind;

use proc_macro::TokenStream;

#[proc_macro_derive(Bind)]
pub fn derive_bind(input: TokenStream) -> TokenStream {
    bind::derive(input)
}
