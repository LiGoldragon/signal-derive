//! `#[derive(Schema)]` — schema-introspection codegen for signal
//! record kinds.
//!
//! Emits an `impl signal::Kind for T` block whose `DESCRIPTOR`
//! const describes the type's shape.
//!
//! **Status: role under review.** Per criome's `ARCHITECTURE.md`
//! §10.2 + §10.3, the runtime authority for "what kinds exist" is
//! sema-resident records, constructed by criome's init at engine
//! boot. The `DESCRIPTOR` consts this macro emits no longer have an
//! obvious consumer. The crate's continuation, repurposing, or
//! retirement is tracked under bd issue `mentci-next-4v6`. The
//! mechanism stays in place pending decision.
//!
//! Field-type mapping:
//!
//! - `String` → `FieldType::Text`
//! - `bool` → `FieldType::Bool`
//! - `u8` / `u16` / `u32` / `u64` / `i8` / `i16` / `i32` / `i64` →
//!   `FieldType::Integer`
//! - `f32` / `f64` → `FieldType::Float`
//! - `Slot<Kind>` → `FieldType::SlotRef { of_kind: "Kind" }`
//! - `Slot<AnyKind>` → `FieldType::AnyKind`
//! - `Vec<T>` → `is_list: true` + the inner T's mapping
//! - `Option<T>` → `is_optional: true` + the inner T's mapping
//! - any other path → `FieldType::Record { kind_name: "..." }`.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, GenericArgument, PathArguments, Type, Variant, parse_macro_input};

/// `#[derive(Schema)]` — emits `impl signal::Kind for T`.
#[proc_macro_derive(Schema)]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();

    let shape: TokenStream2 = match &input.data {
        Data::Struct(data) => struct_shape(data),
        Data::Enum(data) => enum_shape(data),
        Data::Union(_) => {
            return syn::Error::new_spanned(
                name,
                "Schema does not support unions — use a struct or a unit-variant enum",
            )
            .to_compile_error()
            .into();
        }
    };

    let expanded = quote! {
        impl ::signal::Kind for #name {
            const DESCRIPTOR: ::signal::KindDescriptor = ::signal::KindDescriptor {
                name: #name_str,
                shape: #shape,
            };
        }
    };

    expanded.into()
}

fn struct_shape(data: &DataStruct) -> TokenStream2 {
    let fields = match &data.fields {
        Fields::Named(named) => &named.named,
        Fields::Unit => {
            return quote! {
                ::signal::KindShape::Record { fields: &[] }
            };
        }
        Fields::Unnamed(_) => {
            return syn::Error::new_spanned(
                &data.fields,
                "Schema does not support tuple structs — use named fields or a unit struct",
            )
            .to_compile_error();
        }
    };

    let field_descriptors = fields.iter().map(|field| {
        let name = field
            .ident
            .as_ref()
            .expect("named-field struct has named fields")
            .to_string();
        let (field_type, is_optional, is_list) = lower_field_type(&field.ty);
        quote! {
            ::signal::FieldDescriptor {
                name: #name,
                field_type: #field_type,
                is_optional: #is_optional,
                is_list: #is_list,
            }
        }
    });

    quote! {
        ::signal::KindShape::Record {
            fields: &[#(#field_descriptors),*],
        }
    }
}

fn enum_shape(data: &DataEnum) -> TokenStream2 {
    let variants_or_error: Result<Vec<String>, syn::Error> =
        data.variants.iter().map(variant_name_or_error).collect();

    let variant_names = match variants_or_error {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    let variant_str = variant_names.iter().map(|n| quote! { #n });

    quote! {
        ::signal::KindShape::Enum {
            variants: &[#(#variant_str),*],
        }
    }
}

fn variant_name_or_error(variant: &Variant) -> Result<String, syn::Error> {
    match &variant.fields {
        Fields::Unit => Ok(variant.ident.to_string()),
        _ => Err(syn::Error::new_spanned(
            variant,
            "Schema enum variants must be unit (no fields). Data-carrying \
             variants don't fit the closed-vocabulary shape — split them \
             into separate kinds instead.",
        )),
    }
}

/// Lower a `syn::Type` into `(FieldType expression, is_optional, is_list)`.
fn lower_field_type(ty: &Type) -> (TokenStream2, bool, bool) {
    if let Some(inner) = unwrap_path("Option", ty) {
        let (inner_type, _, inner_is_list) = lower_field_type(inner);
        return (inner_type, true, inner_is_list);
    }
    if let Some(inner) = unwrap_path("Vec", ty) {
        let (inner_type, inner_is_optional, _) = lower_field_type(inner);
        return (inner_type, inner_is_optional, true);
    }

    (lower_simple_type(ty), false, false)
}

fn lower_simple_type(ty: &Type) -> TokenStream2 {
    let path = match ty {
        Type::Path(path) => &path.path,
        _ => {
            return syn::Error::new_spanned(
                ty,
                "Schema only handles named types — references, tuples, \
                 arrays, and trait objects aren't part of the record \
                 vocabulary.",
            )
            .to_compile_error();
        }
    };

    let last = match path.segments.last() {
        Some(s) => s,
        None => {
            return syn::Error::new_spanned(path, "empty type path")
                .to_compile_error();
        }
    };

    let ident = last.ident.to_string();

    match ident.as_str() {
        "String" => quote! { ::signal::FieldType::Text },
        "bool" => quote! { ::signal::FieldType::Bool },
        "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => {
            quote! { ::signal::FieldType::Integer }
        }
        "f32" | "f64" => quote! { ::signal::FieldType::Float },
        "Slot" => slot_field_type(&last.arguments, ty),
        _ => {
            // Any other named type — defer resolution to the
            // consumer via the kind name.
            quote! {
                ::signal::FieldType::Record { kind_name: #ident }
            }
        }
    }
}

/// `Slot<Kind>` → `FieldType::SlotRef { of_kind: "Kind" }`,
/// or `FieldType::AnyKind` when `Kind` is `AnyKind`.
fn slot_field_type(args: &PathArguments, ty: &Type) -> TokenStream2 {
    let args = match args {
        PathArguments::AngleBracketed(a) => a,
        _ => {
            return syn::Error::new_spanned(
                ty,
                "Slot must carry one type argument — `Slot<Kind>`.",
            )
            .to_compile_error();
        }
    };

    let kind_arg = match args.args.iter().find_map(|arg| match arg {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }) {
        Some(t) => t,
        None => {
            return syn::Error::new_spanned(
                ty,
                "Slot must carry one type argument — `Slot<Kind>`.",
            )
            .to_compile_error();
        }
    };

    let kind_path = match kind_arg {
        Type::Path(path) => &path.path,
        _ => {
            return syn::Error::new_spanned(
                kind_arg,
                "Slot's kind argument must be a named type — `Slot<Node>`, \
                 `Slot<AnyKind>`, etc.",
            )
            .to_compile_error();
        }
    };

    let kind_ident = match kind_path.segments.last() {
        Some(s) => s.ident.to_string(),
        None => {
            return syn::Error::new_spanned(kind_arg, "empty kind path")
                .to_compile_error();
        }
    };

    if kind_ident == "AnyKind" {
        quote! { ::signal::FieldType::AnyKind }
    } else {
        quote! {
            ::signal::FieldType::SlotRef { of_kind: #kind_ident }
        }
    }
}

/// If `ty` is `Wrapper<Inner>` for the named wrapper, return the
/// inner type. Used for `Option<T>` and `Vec<T>` unwrapping.
fn unwrap_path<'a>(wrapper_name: &str, ty: &'a Type) -> Option<&'a Type> {
    let path = match ty {
        Type::Path(p) => &p.path,
        _ => return None,
    };
    let last = path.segments.last()?;
    if last.ident != wrapper_name {
        return None;
    }
    let args = match &last.arguments {
        PathArguments::AngleBracketed(a) => a,
        _ => return None,
    };
    args.args.iter().find_map(|arg| match arg {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    })
}
