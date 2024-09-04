use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let fields: Vec<Ident> = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Unit => Vec::new(),
            Fields::Named(fs) => fs.named.into_iter().map(|f| f.ident.expect("#[derive(ApproxEq)] requires all fields to be named")).collect(),
            // TODO: Support unnamed fields
            _ => panic!("#[derive(ApproxEq)] requires named fields!"),
        },
        // TODO: Support enums
        _ => panic!("#[derive(ApproxEq)] only works on structs!")
    };

    let impl_block = quote! {
        impl ::nuit::ApproxEq for #name {
            fn approx_eq(&self, other: &Self, tolerance: f64) -> bool {
                #(self.#fields.approx_eq(&other.#fields, tolerance))&&*
            }
        }
    };

    impl_block.into()
}
