extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Meta, Type};

#[proc_macro_derive(ReprTransparent)]
pub fn derive_repr_transparent(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    inner(input).unwrap_or_else(|e| TokenStream::from(e.to_compile_error()))
}

fn inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    check_for_repr_transparent_attribute(&input)?;

    let wrapped_type = get_wrapped_type(&input)?;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        unsafe impl #impl_generics bointer::ReprTransparent for #name #ty_generics #where_clause {
            type Wrapped = #wrapped_type;

            fn into_wrapped(self) -> Self::Wrapped {
                // SAFETY: Safe for the conditions described in the trait documentation.
                unsafe { core::mem::transmute(self) }
            }
        }
    };

    // Hand the output tokens back to the compiler
    Ok(TokenStream::from(expanded))
}

fn check_for_repr_transparent_attribute(input: &DeriveInput) -> Result<(), Error> {
    let repr_attr = input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("repr"))
        .ok_or_else(|| {
            Error::new_spanned(
                input,
                "`#[repr(transparent)]` is required to implement `ReprTransparent`",
            )
        })?;

    match repr_attr.parse_meta()? {
        Meta::List(list) => {
            if !list.nested.iter().any(|nested| match nested {
                syn::NestedMeta::Meta(meta) => meta.path().is_ident("transparent"),
                _ => false,
            }) {
                return Err(Error::new_spanned(
                    list,
                    "`#[repr(transparent)]` is required to implement `ReprTransparent`",
                ));
            }
        }
        bad => {
            return Err(Error::new_spanned(
                bad,
                "`#[repr(transparent)]` is required to implement `ReprTransparent`",
            ))
        }
    }
    Ok(())
}

fn get_wrapped_type(input: &DeriveInput) -> Result<&Type, Error> {
    let fields = match &input.data {
        Data::Struct(struct_data) => &struct_data.fields,
        Data::Enum(enum_data) => {
            &enum_data
                .variants
                .first()
                .ok_or_else(|| {
                    Error::new_spanned(
                        input,
                        "The enum need exactly one variant to implement `ReprTransparent`",
                    )
                })?
                .fields
        }
        Data::Union(_) => {
            return Err(Error::new_spanned(
                input,
                "Unions are not supported by the `ReprTransparent` derive",
            ))
        }
    };

    let ty = match fields {
        Fields::Named(fields) => {
            &fields
                .named
                .first()
                .ok_or_else(|| {
                    Error::new_spanned(fields, "Needs at least one struct/variant field")
                })?
                .ty
        }
        Fields::Unnamed(fields) => {
            &fields
                .unnamed
                .first()
                .ok_or_else(|| {
                    Error::new_spanned(fields, "Needs at least one struct/variant field")
                })?
                .ty
        }
        Fields::Unit => {
            return Err(Error::new_spanned(
                fields,
                "Needs at least one struct/variant field",
            ))
        }
    };

    Ok(ty)
}
