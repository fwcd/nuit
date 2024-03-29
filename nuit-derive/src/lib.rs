use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Type, Field, Ident};

#[proc_macro_derive(Bind)]
pub fn derive_bind(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let state_fields: Vec<Ident> = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Unit => Vec::new(),
            Fields::Named(fs) => fs.named.into_iter()
                .filter(|Field { ty, .. }| is_state_type(&ty))
                .map(|f| f.ident.expect("#[derive(Bind)] requires all state fields to be named"))
                .collect(),
            _ => panic!("#[derive(Bind)] requires named fields!"),
        },
        _ => panic!("#[derive(Bind)] only works on structs!")
    };

    let indices = 0..state_fields.len();

    // TODO: Handle generic structs
    let impl_block = quote! {
        impl ::nuit::Bind for #name {
            fn bind(&mut self, context: &::nuit::Context) {
                #(self.#state_fields.link(context.storage().clone(), context.id_path(), #indices);)*
            }
        }
    };

    impl_block.into()
}

fn is_state_type(ty: &Type) -> bool {
    // TODO: Support qualified/parenthesized/... type annotations that resolve to `nuit::State` too
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
