extern crate proc_macro;

use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use venial::{parse_declaration, Declaration, Error, StructFields};

#[proc_macro_derive(ReprTransparent)]
pub fn derive_repr_transparent(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_declaration(input.into())
        .and_then(derive)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn derive(decl: Declaration) -> Result<TokenStream, Error> {
    let Some(repr_attribute) = decl.attributes().iter().find(|attr| {
        attr.get_single_path_segment()
            .map(|ident| ident == "repr")
            .unwrap_or(false)
    }) else {
        return Err(Error::new_at_tokens(
            decl,
            "This derive only supports types with `#[repr(...)]`",
        ));
    };

    if !matches!(
        repr_attribute.get_value_tokens(),
        [TokenTree::Ident(id)] if id == "transparent"
    ) {
        return Err(Error::new_at_tokens(
            repr_attribute,
            "`#[repr(transparent)]` is required to implement `ReprTransparent`",
        ));
    }

    let (inner_type, inline_generic_args, name, generic_params, where_clause) = match &decl {
        Declaration::Struct(decl) => {
            let inner_type =
                decl.field_types().into_iter().next().ok_or_else(|| {
                    Error::new_at_tokens(decl, "Struct must have at least one field")
                })?;

            (
                inner_type,
                decl.get_inline_generic_args(),
                &decl.name,
                &decl.generic_params,
                &decl.where_clause,
            )
        }
        Declaration::Enum(decl) => {
            let (variant, _) = decl
                .variants
                .first()
                .ok_or_else(|| Error::new_at_tokens(decl, "Enum must have one variant"))?;

            let inner_type = match &variant.contents {
                StructFields::Unit => {
                    return Err(Error::new_at_tokens(
                        variant,
                        "Enum variant must have at least one field",
                    ))
                }
                StructFields::Tuple(f) => f.fields.first().map(|f| &f.0.ty).ok_or_else(|| {
                    Error::new_at_tokens(variant, "Enum variant must have at least one field")
                })?,
                StructFields::Named(f) => f.fields.first().map(|f| &f.0.ty).ok_or_else(|| {
                    Error::new_at_tokens(variant, "Enum variant must have at least one field")
                })?,
            };

            (
                inner_type,
                decl.get_inline_generic_args(),
                &decl.name,
                &decl.generic_params,
                &decl.where_clause,
            )
        }
        _ => return Err(Error::new("This derive only supports structs or enums")),
    };

    // Build the output, possibly using quasi-quotation
    Ok(quote! {
        unsafe impl #inline_generic_args bointer::ReprTransparent for #name #generic_params #where_clause {
            type Inner = #inner_type;

            #[inline(always)]
            fn into_inner(self) -> Self::Inner {
                // SAFETY: Safe for the conditions described in the trait documentation.
                unsafe { core::mem::transmute(self) }
            }
        }
    })
}
