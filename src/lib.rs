use proc_macro2::Ident;
use quote::{quote, quote_spanned};

fn elems_field(ast: &syn::DeriveInput) -> Option<&syn::Field> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = &ast.data
    {
        named.iter().find(|elem| {
            if let Some(ident) = &elem.ident {
                ident == "elems"
            } else {
                false
            }
        })
    } else {
        None
    }
}

fn generate_indexing(struct_ident: Ident, elems_type: Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::ops::Index<(usize, usize)> for #struct_ident {
            type Output = #elems_type;

            fn index(&self, index: (usize, usize)) -> &Self::Output {
                &self.elems[index.1][index.0]
            }
        }
        impl std::ops::Index<usize> for #struct_ident {
            type Output = Vec<#elems_type>;

            fn index(&self, index: usize) -> &Self::Output {
                &self.elems[index]
            }
        }
        impl std::ops::IndexMut<(usize, usize)> for #struct_ident {
            fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
                &mut self.elems[index.1][index.0]
            }
        }
        impl std::ops::IndexMut<usize> for #struct_ident {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.elems[index]
            }
        }
    }
}

fn find_contained_elem_type(ty: &syn::Type) -> Option<syn::Ident> {
    let syn::Type::Path(syn::TypePath { path, .. }) = ty else {
        return None;
    };
    let syn::Path { segments, .. } = path;
    if !segments[0].ident.to_string().ends_with("Vec") {
        return None;
    }
    let syn::PathSegment {
        arguments:
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }),
        ..
    } = &segments[0]
    else {
        return None;
    };
    let syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, .. })) = &args[0] else {
        return None;
    };
    let syn::Path { segments, .. } = path;
    if !segments[0].ident.to_string().ends_with("Vec") {
        return None;
    }
    let syn::PathSegment {
        arguments:
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }),
        ..
    } = &segments[0]
    else {
        return None;
    };
    let syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, .. })) = &args[0] else {
        return None;
    };
    let syn::Path { segments, .. } = path;
    Some(segments[0].ident.clone())
}

fn impl_useful_functs(struct_ident: Ident, elems_type: Ident) -> proc_macro2::TokenStream {
    quote! {
        impl #struct_ident {
            pub fn y_len(&self) -> usize {
                self.elems.len()
            }
            pub fn x_len(&self) -> usize {
                self[0].len()
            }
            pub fn is_within_bounds(&self, pos: (usize, usize)) -> bool {
                // no need to check > 0 because usize
                pos.0 < self.x_len() && pos.1 < self.y_len()
            }
        }
    }
}

#[proc_macro_derive(TwoDArray)]
pub fn make_2d_array(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let Some(elems_field) = elems_field(&ast) else {
        return quote_spanned!(ast.ident.span() => compile_error!("Must contain elems field");)
            .into();
    };
    let elems_type = &elems_field.ty;
    let contained_type = find_contained_elem_type(elems_type).unwrap();
    let index_impls = generate_indexing(ast.ident.clone(), contained_type.clone());
    let useful_functions = impl_useful_functs(ast.ident.clone(), contained_type);
    quote! {
        #index_impls
        #useful_functions
    }
    .into()
}
