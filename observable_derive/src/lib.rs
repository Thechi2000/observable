use proc_macro::TokenStream;

use proc_macro2::Ident;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{Item, parse_macro_input};

#[proc_macro_derive(Observable)]
pub fn derive_observable(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    let tokens =
        if let Item::Struct(item) = input {
            let variants =
                item.fields.into_iter()
                    .map(|f| {
                        let name = f.ident.unwrap_or_else(|| Ident::new("_", Span::call_site()));
                        let ty = f.ty.into_token_stream();

                        quote! { #name (#ty, #ty),}
                    })
                    .fold(quote! {}, |t1, t2| quote! {#t1 #t2});

            let struct_name = item.ident;
            let enum_name = format_ident!("{}MutationEvent", struct_name);
            quote! {
                pub enum #enum_name{ #variants };
                impl Observable for #struct_name {
                    type Item = #struct_name;
                    type MutationEvent = #enum_name;

                    fn update(&mut self, _: Self::Item)-> Vec<Self::MutationEvent>{
                        panic!();
                        vec![]
                    }
                }
            }
        } else {
            quote! {}
        };

    dbg!(tokens.to_string());
    tokens.into()
}