//! Code generation for projects using iroh.

use convert_case::{Case, Casing};
use proc_macro::{self, TokenStream};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, FieldsNamed, FieldsUnnamed, Index, Type};

/// Generate lenses for each field in a struct
/// Currently, this doesn't support enums or unions.
#[proc_macro_derive(Lens)]
pub fn lens(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: struct_name,
        data,
        ..
    } = parse_macro_input!(input);

    let descs = match data {
        syn::Data::Struct(s) => {
            let idents = match s.fields {
                syn::Fields::Named(FieldsNamed { named, .. }) => named
                    .iter()
                    .map(|f| (Box::new(f.ident.clone()) as Box<dyn ToTokens>, f.ty.clone()))
                    .collect(),
                syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, f)| (Box::new(Index::from(i)) as Box<dyn ToTokens>, f.ty.clone()))
                    .collect(),
                syn::Fields::Unit => vec![],
            };

            idents
                .into_iter()
                .map(|(i, ty)| LensDesc::from_field(i, ty, struct_name.clone()))
        }
        syn::Data::Enum(_) => {
            panic!("Cannot derive lenses for an enum")
        }
        syn::Data::Union(_) => {
            todo!()
        }
    };

    let lenses = descs.map(|d| d.into_lens());

    let output = quote! {
    #(#lenses)*
    };

    output.into()
}

struct LensDesc {
    field: Box<dyn ToTokens>,
    ty: Type,
    name: Ident,
    target: Ident,
}

impl LensDesc {
    fn from_field(field: Box<dyn ToTokens>, ty: Type, struct_name: Ident) -> Self {
        let lens_name = format_ident!(
            "{}{}Lens",
            struct_name,
            format!("{}", field.to_token_stream()).to_case(Case::Pascal)
        );
        Self {
            field,
            ty,
            name: lens_name,
            target: struct_name,
        }
    }

    fn into_lens(self) -> TokenStream2 {
        let LensDesc {
            field,
            ty,
            name,
            target,
        } = self;
        quote! {
            #[derive(Debug, Clone)]
            pub struct #name;
            impl iroh::lens::Lens for #name {
                type Source = #target;
                type Target = #ty;

                fn get<'a>(source: &'a Self::Source) -> &'a Self::Target {
                    &source.#field
                }
                fn get_mut<'a>(source: &'a mut Self::Source) -> &'a mut Self::Target {
                    &mut source.#field
                }
            }
        }
    }
}
