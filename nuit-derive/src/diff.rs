use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident, Type, TypeParam, Variant};

pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let type_params: Vec<TypeParam> = input.generics.type_params().cloned().collect();

    let variants: Vec<Variant> = match input.data {
        Data::Enum(e) => e.variants.into_iter().collect(),
        _ => panic!("#[derive(Diff)] only works on enums!")
    };

    let impl_block = quote! {
        impl<#(#type_params),*> ::nuit::Diff for #name<#(#type_params),*> {
            fn record_diff<'a>(&'a self, old: &'a Self, id_path: &::nuit::IdPath, difference: &mut ::nuit::Difference<(::nuit::IdPathBuf, &'a Self)>) {
                use ::std::mem;

                if mem::discriminant(self) != mem::discriminant(old) {
                    difference.removed.push((id_path.to_owned(), old));
                    difference.added.push((id_path.to_owned(), self));
                }

                // TODO
            }
        }
    };

    impl_block.into()
}

fn is_identified_type(ty: &Type) -> bool {
    // TODO: Support qualified/parenthesized/... type annotations that resolve to `nuit::Identified` too
    match ty {
        Type::Path(ty_path) => {
            let segments = &ty_path.path.segments;
            if let Some(segment) = segments.first() {
                segment.ident.to_string() == "Identified"
            } else {
                false
            }
        },
        _ => false
    }
}
