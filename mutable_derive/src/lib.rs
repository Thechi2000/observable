mod derive_mutable;
mod derive_softeq;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_derive(Mutable)]
#[proc_macro_error]
pub fn derive_mutable(item: TokenStream) -> TokenStream {
    if let Ok(ts) = derive_mutable::try_(item) {
        ts
    } else {
        panic!()
    }
}

#[proc_macro_derive(SoftEq, attributes(softeq))]
#[proc_macro_error]
pub fn derive_softeq(item: TokenStream) -> TokenStream {
    if let Ok(ts) = derive_softeq::try_(item) {
        ts
    } else {
        panic!()
    }
}