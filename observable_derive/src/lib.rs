mod derive_observable;
mod derive_softeq;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_derive(Observable)]
#[proc_macro_error]
pub fn derive_observable(item: TokenStream) -> TokenStream {
    if let Ok(ts) = derive_observable::try_(item) {
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