use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident, Type};

pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let const_params: Vec<_> = input.generics.const_params().collect();
    let const_idents: Vec<_> = input.generics.const_params().map(|p| p.ident.clone()).collect();
    let type_params: Vec<_> = input.generics.type_params().collect();

    let state_fields: Vec<Ident> = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Unit => Vec::new(),
            Fields::Named(fs) => fs.named.into_iter()
                .filter(|Field { ty, .. }| is_state_type(ty))
                .map(|f| f.ident.expect("#[derive(Bind)] requires all state fields to be named"))
                .collect(),
            _ => panic!("#[derive(Bind)] requires named fields!"),
        },
        _ => panic!("#[derive(Bind)] only works on structs!")
    };

    let indices = 0..state_fields.len();

    let impl_block = quote! {
        impl <#(#const_params,)* #(#type_params,)*> ::nuit::Bind for #name <#(#const_idents,)* #(#type_params,)*> {
            fn bind(&self, context: &::nuit::Context) {
                #(self.#state_fields.link(context.storage().clone(), ::nuit::StateKey::new(context.id_path(), #indices));)*
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
                segment.ident == "State"
            } else {
                false
            }
        },
        _ => false
    }
}
