use proc_macro::{TokenStream, Span};
use convert_case::{Case, Casing};
use proc_macro_error::abort;
use quote::ToTokens;
use syn::Item;

pub(crate) fn try_(item: TokenStream) -> Result<TokenStream, ()> {
    let item = syn::parse::<Item>(item).map_err(|_| abort!(Span::call_site(), "Could not parse item"))?;

    let struct_ = match item {
        Item::Struct(struct_) => struct_,
        _ => abort!(item, "Observable may only be derived on structures"),
    };

    let (mutation_checks, mutation_variants): (Vec<String>, Vec<String>) = struct_.fields.iter().map(|field| {
        (format!(r#"updates.append(&mut <{field_type} as observable::Observable>::cmp(&self.{field_name}, &new_value.{field_name}).into_iter().map(Self::Mutation::{mutation_name}).collect())"#,
                 field_type = field.ty.to_token_stream(),
                 field_name = field.ident.as_ref().unwrap(),
                 mutation_name = field.ident.as_ref().unwrap().to_string().to_case(Case::Pascal)),
         format!(r#"{}(<{} as observable::Observable>::Mutation)"#,
                 field.ident.as_ref().unwrap().to_string().to_case(Case::Pascal),
                 field.ty.to_token_stream()
         ))
    }).collect::<Vec<(String, String)>>().into_iter().unzip();

    let observable_impl = format!(
        r#"
        #[derive(std::fmt::Debug, std::cmp::PartialEq)]
        enum {struct_name}Mutation{{
            {mutation_variants}
        }}

        impl observable::Observable for {struct_name}{{
            type Mutation={struct_name}Mutation;

            fn cmp(&self, new_value: &Self) -> Vec<Self::Mutation>{{
                let mut updates: Vec<Self::Mutation> = Vec::new();
                {mutation_checks};
                updates
            }}
        }}"#,
        struct_name= struct_.ident,
        mutation_variants=mutation_variants.join(",\n"),
        mutation_checks=mutation_checks.join(";\n"),
    );

    Ok(observable_impl.parse().unwrap())
}