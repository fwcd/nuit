use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, GenericArgument, Ident, PathArguments, Type, Variant};

pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let variants: Vec<Variant> = match input.data {
        Data::Enum(e) => e.variants.into_iter().collect(),
        _ => panic!("#[derive(Diff)] only works on enums!")
    };

    let variant_arms: Vec<proc_macro2::TokenStream> = create_variant_match_arms(variants, name);

    let impl_block = quote! {
        impl ::nuit::Diff for #name {
            fn record_diff<'a>(&'a self, old: &'a Self, id_path: &::nuit::IdPath, difference: &mut ::nuit::Difference<&'a Self>) {
                use ::std::{mem, collections::{HashSet, HashMap}};
                use ::nuit::{Difference, Identified, Id, IdPath};

                fn recurse_on<'a>(new_child: &'a Identified<#name>, old_child: &'a Identified<#name>, id_path: &IdPath, difference: &mut Difference<&'a #name>) {
                    if new_child.id() != old_child.id() {
                        difference.removed.push((id_path.child(old_child.id().clone()), old_child.value()));
                        difference.added.push((id_path.child(new_child.id().clone()), new_child.value()));
                        return;
                    }
                    new_child.value().record_diff(old_child.value(), &id_path.child(new_child.id().clone()), difference);
                }

                fn recurse_on_container<'a>(new_child_container: &'a Vec<Identified<#name>>, old_child_container: &'a Vec<Identified<#name>>, id_path: &IdPath, difference: &mut Difference<&'a #name>) {
                    let new_children: HashMap<&Id, &Identified<#name>> = new_child_container.iter().map(|c| (c.id(), c)).collect();
                    let old_children: HashMap<&Id, &Identified<#name>> = old_child_container.iter().map(|c| (c.id(), c)).collect();

                    let new_ids: HashSet<&Id> = new_children.keys().cloned().collect();
                    let old_ids: HashSet<&Id> = old_children.keys().cloned().collect();

                    for &child_id in new_ids.difference(&old_ids) {
                        difference.added.push((id_path.child(child_id.clone()), new_children[child_id].value()));
                    }

                    for &child_id in old_ids.difference(&new_ids) {
                        difference.removed.push((id_path.child(child_id.clone()), old_children[child_id].value()));
                    }

                    for &child_id in new_ids.intersection(&old_ids) {
                        recurse_on(new_children[child_id], old_children[child_id], id_path, difference);
                    }
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

fn create_variant_match_arms(variants: Vec<Variant>, self_ty_name: &Ident) -> Vec<proc_macro2::TokenStream> {
    variants.into_iter().map(|v| {
        let ident = v.ident;
        let fields: Vec<(Ident, DiffFieldKind)> = match v.fields {
            Fields::Named(fs) => fs.named.into_iter()
                .map(|f| (f.ident.expect("#[derive(Diff)] currently requires fields to be named"), DiffFieldKind::from_type(f.ty, &self_ty_name)))
                .collect(),
            // TODO: Support unnamed/unit fields
            _ => panic!("#[derive(Diff)] currently only supports enum variants with named fields"),
        };
        let field_idents: Vec<&Ident> = fields.iter().map(|(fi, _)| fi).collect();

        let create_pattern = |i: usize| {
            let bound_vars: Vec<_> = fields.iter().map(|(fi, k)| (format_ident!("{}{}", fi, i), k)).collect();
            let bound_idents: Vec<Ident> = bound_vars.iter().map(|(bi, _)| bi.clone()).collect();

            let pattern = quote! { Self::#ident { #(#field_idents: #bound_idents),* } };
            let simple_idents: Vec<Ident> = bound_vars.iter().filter(|(_, k)| matches!(k, DiffFieldKind::Simple)).map(|(bi, _)| bi.clone()).collect();
            let child_idents: Vec<Ident> = bound_vars.iter().filter(|(_, k)| matches!(k, DiffFieldKind::Child)).map(|(bi, _)| bi.clone()).collect();
            let child_container_idents: Vec<Ident> = bound_vars.iter().filter(|(_, k)| matches!(k, DiffFieldKind::ChildContainer)).map(|(bi, _)| bi.clone()).collect();

            (pattern, simple_idents, child_idents, child_container_idents)
        };

        let (pattern1, simple_idents1, child_idents1, child_container_idents1) = create_pattern(1);
        let (pattern2, simple_idents2, child_idents2, child_container_idents2) = create_pattern(2);

        quote! {
            (#pattern1, #pattern2) => {
                // Handle simple variables by just testing for (in)equality
                if false #(|| #simple_idents1 != #simple_idents2)* {
                    difference.changed.push((id_path.to_owned(), self, old));
                }

                // Handle child (`Identified<Self>`) variables by recursing on them
                #(recurse_on(#child_idents1, #child_idents2, id_path, difference);)*

                // Handle child container (`Vec<Identified<Self>>`) variables by diffing, then recursing
                #(recurse_on_container(#child_container_idents1, #child_container_idents2, id_path, difference);)*
            }
        }
    }).collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum DiffFieldKind {
    Simple,
    Child,
    ChildContainer,
}

const IDENTIFIED: &str = "Identified";
const SELF: &str = "Self";
const BOX: &str = "Box";
const VEC: &str = "Vec";

impl DiffFieldKind {
    fn from_type(ty: Type, self_ty_name: &Ident) -> Self {
        let node = TypeNode::from(ty).normalize_self(self_ty_name.to_string().as_str());

        if node == (BOX, [(IDENTIFIED, [SELF])]).into() { // Identified<Self>
            Self::Child
        } else if node == (VEC, [(IDENTIFIED, [SELF])]).into() { // Vec<Identified<Self>>
            Self::ChildContainer
        } else {
            Self::Simple
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TypeNode(String, Vec<TypeNode>);

impl TypeNode {
    fn normalize_self(self, self_ty_name: &str) -> Self {
        let name = if &self.0 == self_ty_name { SELF.to_owned() } else { self.0 };
        let args = self.1.into_iter().map(|t| t.normalize_self(self_ty_name)).collect();
        Self(name, args)
    }
}

impl<'a> From<&'a str> for TypeNode {
    fn from(name: &'a str) -> Self {
        Self(name.to_string(), Vec::new())
    }
}

impl<'a> From<&'a Ident> for TypeNode {
    fn from(name: &'a Ident) -> Self {
        Self(name.to_string(), Vec::new())
    }
}

impl<S, C> From<(S, C)> for TypeNode where S: ToString, C: IntoIterator, C::Item: Into<TypeNode> {
    fn from((name, args): (S, C)) -> Self {
        Self(name.to_string(), args.into_iter().map(|i| i.into()).collect())
    }
}

impl From<Type> for TypeNode {
    fn from(ty: Type) -> Self {
        // TODO: Support qualified/parenthesized/... types
        match ty {
            Type::Path(ty_path) => {
                let segments = ty_path.path.segments;
                if let Some(segment) = segments.into_iter().last() {
                    let name = segment.ident.to_string();
                    let args = match segment.arguments {
                        PathArguments::AngleBracketed(args) => args.args.into_iter()
                            .filter_map(|a| match a {
                                GenericArgument::Type(t) => Some(Self::from(t)),
                                _ => None,
                            })
                            .collect(),
                        _ => Vec::new(),
                    };
                    return Self(name, args)
                }
            },
            _ => {}
        }
        Self(String::new(), Vec::new())
    }
}
