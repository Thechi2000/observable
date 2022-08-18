use proc_macro::{TokenStream, Span};
use proc_macro_error::abort;
use quote::ToTokens;
use syn::{Item, Meta, NestedMeta};

pub(crate) fn try_(item: TokenStream) -> Result<TokenStream, ()> {
    let item = syn::parse::<Item>(item).map_err(|_| abort!(Span::call_site(), "Could not parse item"))?;

    let struct_ = match item {
        Item::Struct(struct_) => struct_,
        _ => abort!(item, "Observable may only be derived on structures"),
    };

    let (uid_types, uid_names): (Vec<String>, Vec<String>) = struct_.fields.iter()
        .filter(|field| if let Some(meta) = field.attrs.iter().find(|a| a.path.is_ident("softeq")) {
            if let Ok(Meta::List(list)) = meta.parse_meta() {
                list.nested.iter().any(|m| {
                    if let NestedMeta::Meta(Meta::Path(p)) = m {
                        p.is_ident("uid")
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        } else {
            false
        })
        .map(|field| (field.ty.to_token_stream().to_string(), field.ident.as_ref().unwrap().to_string()))
        .unzip();

    if uid_types.is_empty() {
        abort!(struct_, "SoftEq requires at least one unique identifier");
    }

    Ok(format!(r#"
            impl SoftEq for {}{{
                type Uid = ({});

                fn se(&self, other: &Self) -> bool{{
                    {}
                }}
                fn uid(&self) -> Self::Uid{{
                    ({})
                }}
            }}
            "#,
               struct_.ident,
               uid_types.join(", "),
               uid_names.iter().map(|n| format!("self.{n} == other.{n}")).collect::<Vec<String>>().join("&&"),
               uid_names.iter().map(|n| format!("self.{n}.clone()")).collect::<Vec<String>>().join(", ")
    ).parse().unwrap())
}