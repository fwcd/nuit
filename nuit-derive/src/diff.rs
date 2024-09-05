use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Type, Variant};

pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let variants: Vec<Variant> = match input.data {
        Data::Enum(e) => e.variants.into_iter().collect(),
        _ => panic!("#[derive(Diff)] only works on enums!")
    };

    let variant_arms: Vec<proc_macro2::TokenStream> = create_variant_match_arms(variants);

    let impl_block = quote! {
        impl ::nuit::Diff for #name {
            fn record_diff<'a>(&'a self, old: &'a Self, id_path: &::nuit::IdPath, difference: &mut ::nuit::Difference<(::nuit::IdPathBuf, &'a Self)>) {
                use ::std::mem;

                fn recurse_on<'a>(new_child: &'a Identified<#name>, old_child: &'a Identified<#name>, id_path: &::nuit::IdPath, difference: &mut ::nuit::Difference<(::nuit::IdPathBuf, &'a #name)>) {
                    if new_child.id() != old_child.id() {
                        difference.removed.push((id_path.child(old_child.id().clone()), old_child.value()));
                        difference.added.push((id_path.child(new_child.id().clone()), new_child.value()));
                        return;
                    }
                    new_child.value().record_diff(old_child.value(), &id_path.child(new_child.id().clone()), difference);
                }

                if mem::discriminant(self) != mem::discriminant(old) {
                    difference.removed.push((id_path.to_owned(), old));
                    difference.added.push((id_path.to_owned(), self));
                }

                match (self, old) {
                    #(#variant_arms,)*
                    _ => unreachable!(),
                }
            }
        }
    };

    impl_block.into()
}

fn create_variant_match_arms(variants: Vec<Variant>) -> Vec<proc_macro2::TokenStream> {
    variants.into_iter().map(|v| {
        let ident = v.ident;
        let field_idents: Vec<Ident> = match v.fields {
            Fields::Named(fs) => fs.named.into_iter()
                .map(|f| f.ident.expect("#[derive(Diff)] currently requires fields to be named"))
                .collect(),
            // TODO: Support unnamed/unit fields
            _ => panic!("#[derive(Diff)] currently only supports enum variants with named fields"),
        };

        let create_pattern = |i: usize| {
            let bound_idents: Vec<_> = field_idents.iter().map(|fi| format_ident!("{}{}", fi, i)).collect();
            (quote! { Self::#ident { #(#field_idents: #bound_idents),* } }, bound_idents)
        };

        let (pattern1, bound_idents1) = create_pattern(1);
        let (pattern2, bound_idents2) = create_pattern(2);

        quote! {
            (#pattern1, #pattern2) => {
                if false #(|| #bound_idents1 != #bound_idents2)* {
                    difference.changed.push((id_path.to_owned(), self));
                }
            }
        }
    }).collect()
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
