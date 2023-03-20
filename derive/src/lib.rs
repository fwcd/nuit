use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Type, Field, Ident};

#[proc_macro_derive(Bind)]
pub fn derive_bind(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let state_fields: Vec<Ident> = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fs) => fs.named.into_iter()
                .filter(|Field { ty, .. }| is_state_type(&ty))
                .map(|f| f.ident.expect("#[derive(Bind)] requires all state fields to be named"))
                .collect(),
            _ => panic!("#[derive(Bind)] requires named fields!"),
        },
        _ => panic!("#[derive(Bind)] only works on structs!")
    };

    let link_calls = state_fields
        .into_iter()
        .enumerate()
        .map(|(i, name)| quote! {
            (self.#name).link(context.storage().clone(), context.id_path().clone(), #i);
        });

    // TODO: Handle generic structs
    let impl_block = quote! {
        impl nui::Bind for #name {
            fn bind(&mut self, context: &nui::Context) {
                #(#link_calls)*
            }
        }
    };

    return impl_block.into();
}

fn is_state_type(ty: &Type) -> bool {
    // TODO: Support qualified/parenthesized/... type annotations that resolve to `nui::State` too
    match ty {
        Type::Path(ty_path) => {
            let segments = &ty_path.path.segments;
            if let Some(segment) = segments.first() {
                segment.ident.to_string() == "State"
            } else {
                false
            }
        },
        _ => false
    }
}
